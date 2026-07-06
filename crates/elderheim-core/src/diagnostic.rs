use core::fmt::{self, Write};

use crate::{LineColumn, LineCursor, Source, SourceError, SourceId, Span};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticCode {
    UnsupportedFeature,
    InvalidDialect,
    ProgramTooLarge,
    InvalidExecutableLayout,
    SourceTooLarge,
    TooManyLines,
    InvalidSpan,
    SourceOffsetOutOfBounds,
    InvalidSourceByte,
    BlankSourceLine,
    InternalLocation,
}

impl DiagnosticCode {
    pub const ALL: &'static [Self] = &[
        Self::UnsupportedFeature,
        Self::InvalidDialect,
        Self::ProgramTooLarge,
        Self::InvalidExecutableLayout,
        Self::SourceTooLarge,
        Self::TooManyLines,
        Self::InvalidSpan,
        Self::SourceOffsetOutOfBounds,
        Self::InvalidSourceByte,
        Self::BlankSourceLine,
        Self::InternalLocation,
    ];

    #[must_use]
    pub const fn descriptor(self) -> DiagnosticDescriptor {
        DiagnosticDescriptor {
            code: self,
            identifier: self.code(),
            message: self.message(),
            default_severity: Severity::Error,
        }
    }

    #[must_use]
    pub const fn variant_count() -> usize {
        match Self::UnsupportedFeature {
            Self::UnsupportedFeature
            | Self::InvalidDialect
            | Self::ProgramTooLarge
            | Self::InvalidExecutableLayout
            | Self::SourceTooLarge
            | Self::TooManyLines
            | Self::InvalidSpan
            | Self::SourceOffsetOutOfBounds
            | Self::InvalidSourceByte
            | Self::BlankSourceLine
            | Self::InternalLocation => 11,
        }
    }

    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::UnsupportedFeature => "E-CORE-UNSUPPORTED-FEATURE",
            Self::InvalidDialect => "E-CORE-INVALID-DIALECT",
            Self::ProgramTooLarge => "E-CORE-PROGRAM-SIZE",
            Self::InvalidExecutableLayout => "E-CORE-EXECUTABLE-LAYOUT",
            Self::SourceTooLarge => "E-CORE-SOURCE-SIZE",
            Self::TooManyLines => "E-CORE-SOURCE-LINES",
            Self::InvalidSpan => "E-CORE-SOURCE-SPAN",
            Self::SourceOffsetOutOfBounds => "E-CORE-SOURCE-OFFSET",
            Self::InvalidSourceByte => "E-CORE-SOURCE-BYTE",
            Self::BlankSourceLine => "E-CORE-SOURCE-BLANK-LINE",
            Self::InternalLocation => "E-CORE-INTERNAL-LOCATION",
        }
    }

    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::UnsupportedFeature => "feature is not supported by the selected compiler path",
            Self::InvalidDialect => "selected language dialect is not recognized",
            Self::ProgramTooLarge => "program exceeds configured compile limits",
            Self::InvalidExecutableLayout => "executable layout is invalid",
            Self::SourceTooLarge => "source exceeds configured byte limit",
            Self::TooManyLines => "source exceeds configured line limit",
            Self::InvalidSpan => "source span is invalid",
            Self::SourceOffsetOutOfBounds => "source offset is outside the source byte range",
            Self::InvalidSourceByte => "source contains a byte outside the source policy",
            Self::BlankSourceLine => "source contains a blank line rejected by policy",
            Self::InternalLocation => "diagnostic location could not be resolved",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticDescriptor {
    pub code: DiagnosticCode,
    pub identifier: &'static str,
    pub message: &'static str,
    pub default_severity: Severity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

impl Severity {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub severity: Severity,
    pub span: Span,
    pub source_id: Option<SourceId>,
}

impl Diagnostic {
    #[must_use]
    pub const fn new(code: DiagnosticCode, severity: Severity, span: Span) -> Self {
        Self {
            code,
            severity,
            span,
            source_id: None,
        }
    }

    #[must_use]
    pub const fn error(code: DiagnosticCode, span: Span) -> Self {
        Self::new(code, Severity::Error, span)
    }

    #[must_use]
    pub const fn with_source_id(self, source_id: SourceId) -> Self {
        Self {
            source_id: Some(source_id),
            ..self
        }
    }

    pub fn render<W: Write>(
        self,
        source: Option<Source<'_>>,
        style: RenderStyle,
        writer: &mut W,
    ) -> fmt::Result {
        match style {
            RenderStyle::Compact => render_compact(self, source, writer),
            RenderStyle::Snippet => render_snippet(self, source, writer),
        }
    }

    pub fn render_with_cursor<W: Write>(
        self,
        source: Source<'_>,
        cursor: &mut LineCursor,
        style: RenderStyle,
        writer: &mut W,
    ) -> fmt::Result {
        match style {
            RenderStyle::Compact => render_compact_with_cursor(self, source, cursor, writer),
            RenderStyle::Snippet => render_snippet_with_cursor(self, source, cursor, writer),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderStyle {
    Compact,
    Snippet,
}

impl From<SourceError> for DiagnosticCode {
    fn from(value: SourceError) -> Self {
        match value {
            SourceError::SourceTooLarge => Self::SourceTooLarge,
            SourceError::TooManyLines => Self::TooManyLines,
            SourceError::LimitTooLarge => Self::ProgramTooLarge,
            SourceError::OffsetOutOfBounds => Self::SourceOffsetOutOfBounds,
            SourceError::InvalidSpan => Self::InvalidSpan,
            SourceError::LocationOverflow => Self::ProgramTooLarge,
            SourceError::InvalidByte { .. } => Self::InvalidSourceByte,
            SourceError::BlankLine { .. } => Self::BlankSourceLine,
        }
    }
}

fn render_compact<W: Write>(
    diagnostic: Diagnostic,
    source: Option<Source<'_>>,
    writer: &mut W,
) -> fmt::Result {
    if source.is_some_and(|value| !diagnostic.matches_source(value)) {
        return render_location_error(writer);
    }

    match source.map(|value| value.line_column(diagnostic.span.start())) {
        None => render_compact_at(diagnostic, LineColumn { line: 0, column: 0 }, writer),
        Some(Ok(location)) => render_compact_at(diagnostic, location, writer),
        Some(Err(_)) => render_location_error(writer),
    }
}

fn render_compact_with_cursor<W: Write>(
    diagnostic: Diagnostic,
    source: Source<'_>,
    cursor: &mut LineCursor,
    writer: &mut W,
) -> fmt::Result {
    if !diagnostic.matches_source(source) {
        return render_location_error(writer);
    }

    match source.line_column_from(cursor, diagnostic.span.start()) {
        Ok(location) => render_compact_at(diagnostic, location, writer),
        Err(_) => render_location_error(writer),
    }
}

fn render_snippet<W: Write>(
    diagnostic: Diagnostic,
    source: Option<Source<'_>>,
    writer: &mut W,
) -> fmt::Result {
    let Some(source) = source else {
        return render_compact_at(diagnostic, LineColumn { line: 0, column: 0 }, writer);
    };

    if !diagnostic.matches_source(source) {
        return render_location_error(writer);
    }

    let mut cursor = LineCursor::new();
    match source.line_column_from(&mut cursor, diagnostic.span.start()) {
        Ok(location) => {
            render_snippet_at(diagnostic, source, location, cursor.line_start(), writer)
        }
        Err(_) => render_location_error(writer),
    }
}

fn render_snippet_with_cursor<W: Write>(
    diagnostic: Diagnostic,
    source: Source<'_>,
    cursor: &mut LineCursor,
    writer: &mut W,
) -> fmt::Result {
    if !diagnostic.matches_source(source) {
        return render_location_error(writer);
    }

    match source.line_column_from(cursor, diagnostic.span.start()) {
        Ok(location) => {
            render_snippet_at(diagnostic, source, location, cursor.line_start(), writer)
        }
        Err(_) => render_location_error(writer),
    }
}

impl Diagnostic {
    fn matches_source(self, source: Source<'_>) -> bool {
        match self.source_id {
            Some(source_id) => source_id == source.id(),
            None => true,
        }
    }
}

fn render_snippet_at<W: Write>(
    diagnostic: Diagnostic,
    source: Source<'_>,
    location: LineColumn,
    line_start: usize,
    writer: &mut W,
) -> fmt::Result {
    render_compact_at(diagnostic, location, writer)?;
    writeln!(writer, " --> {}:{}", location.line, location.column)?;
    writeln!(writer, "  |")?;
    write!(writer, "{} | ", location.line)?;
    write_source_line(source, line_start, writer)?;
    writeln!(writer)?;
    write!(writer, "  | ")?;
    write_caret_padding(location.column, writer)?;
    writeln!(writer, "^")
}

fn render_compact_at<W: Write>(
    diagnostic: Diagnostic,
    location: LineColumn,
    writer: &mut W,
) -> fmt::Result {
    writeln!(
        writer,
        "{} {} {}:{} {}",
        diagnostic.severity.label(),
        diagnostic.code.code(),
        location.line,
        location.column,
        diagnostic.code.message()
    )
}

fn render_location_error<W: Write>(writer: &mut W) -> fmt::Result {
    render_compact_at(
        Diagnostic::error(DiagnosticCode::InternalLocation, Span::point(0)),
        LineColumn { line: 0, column: 0 },
        writer,
    )
}

fn write_source_line<W: Write>(
    source: Source<'_>,
    line_start: usize,
    writer: &mut W,
) -> fmt::Result {
    let Some(bytes) = source.bytes().get(line_start..) else {
        return Ok(());
    };

    for byte in bytes {
        if *byte == b'\n' {
            return Ok(());
        }
        writer.write_char(char::from(*byte))?;
    }
    Ok(())
}

fn write_caret_padding<W: Write>(column: u32, writer: &mut W) -> fmt::Result {
    let mut remaining = column.saturating_sub(1);
    while remaining > 0 {
        writer.write_char(' ')?;
        remaining = remaining.saturating_sub(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
