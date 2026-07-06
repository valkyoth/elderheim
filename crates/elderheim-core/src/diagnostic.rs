use core::fmt::{self, Write};

use crate::{LineColumn, LineCursor, Source, SourceError, Span};

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
}

impl Diagnostic {
    #[must_use]
    pub const fn new(code: DiagnosticCode, severity: Severity, span: Span) -> Self {
        Self {
            code,
            severity,
            span,
        }
    }

    #[must_use]
    pub const fn error(code: DiagnosticCode, span: Span) -> Self {
        Self::new(code, Severity::Error, span)
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

    match source.line_column(diagnostic.span.start()) {
        Ok(location) => render_snippet_at(diagnostic, source, location, writer),
        Err(_) => render_location_error(writer),
    }
}

fn render_snippet_with_cursor<W: Write>(
    diagnostic: Diagnostic,
    source: Source<'_>,
    cursor: &mut LineCursor,
    writer: &mut W,
) -> fmt::Result {
    match source.line_column_from(cursor, diagnostic.span.start()) {
        Ok(location) => render_snippet_at(diagnostic, source, location, writer),
        Err(_) => render_location_error(writer),
    }
}

fn render_snippet_at<W: Write>(
    diagnostic: Diagnostic,
    source: Source<'_>,
    location: LineColumn,
    writer: &mut W,
) -> fmt::Result {
    render_compact_at(diagnostic, location, writer)?;
    writeln!(writer, " --> {}:{}", location.line, location.column)?;
    writeln!(writer, "  |")?;
    write!(writer, "{} | ", location.line)?;
    write_source_line(source, location.line, writer)?;
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

fn write_source_line<W: Write>(source: Source<'_>, line: u32, writer: &mut W) -> fmt::Result {
    let mut current_line = 1_u32;
    for byte in source.bytes() {
        if current_line == line {
            if *byte == b'\n' {
                return Ok(());
            }
            writer.write_char(char::from(*byte))?;
        } else if *byte == b'\n' {
            current_line = match current_line.checked_add(1) {
                Some(next) => next,
                None => return Ok(()),
            };
        }
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
