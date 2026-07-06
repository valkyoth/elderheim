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
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderStyle {
    Compact,
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::String;

    use super::{Diagnostic, DiagnosticCode, RenderStyle, Severity};
    use crate::{CompileLimits, NormalizationPolicy, Source, SourceError, Span, SpanError};

    fn source(bytes: &[u8]) -> Result<Source<'_>, SourceError> {
        Source::from_normalized(
            bytes,
            CompileLimits::DEFAULT,
            NormalizationPolicy::PRESERVE_BLANK_LINES,
        )
    }

    #[test]
    fn source_errors_map_to_stable_diagnostic_codes() {
        assert_eq!(
            DiagnosticCode::from(SourceError::TooManyLines).code(),
            "E-CORE-SOURCE-LINES"
        );
        assert_eq!(
            DiagnosticCode::from(SourceError::InvalidByte {
                offset: 7,
                byte: 0xff,
            })
            .code(),
            "E-CORE-SOURCE-BYTE"
        );
        assert_eq!(
            DiagnosticCode::from(SourceError::BlankLine { line: 2 }).code(),
            "E-CORE-SOURCE-BLANK-LINE"
        );
    }

    #[test]
    fn diagnostic_compact_rendering_is_golden() -> Result<(), SpanError> {
        let diagnostic = Diagnostic::new(
            DiagnosticCode::UnsupportedFeature,
            Severity::Error,
            Span::checked(9, 15)?,
        );
        let mut rendered = String::new();
        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| diagnostic
                .render(Some(value), RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-UNSUPPORTED-FEATURE 2:1 feature is not supported by the selected compiler path\n"
        );
        Ok(())
    }

    #[test]
    fn diagnostic_without_source_uses_zero_location() {
        let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(0));
        let mut rendered = String::new();
        assert_eq!(
            diagnostic.render(None, RenderStyle::Compact, &mut rendered),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-INVALID-DIALECT 0:0 selected language dialect is not recognized\n"
        );
    }

    #[test]
    fn diagnostic_cursor_rendering_is_golden() -> Result<(), SpanError> {
        let mut cursor = crate::LineCursor::new();
        let mut rendered = String::new();
        let first = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(0));
        let second = Diagnostic::error(DiagnosticCode::UnsupportedFeature, Span::checked(20, 26)?);

        assert_eq!(
            source(b"10 PRINT\n20 GOTO 10\n30 END").and_then(|value| first
                .render_with_cursor(value, &mut cursor, RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            source(b"10 PRINT\n20 GOTO 10\n30 END").and_then(|value| second
                .render_with_cursor(value, &mut cursor, RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-INVALID-DIALECT 1:1 selected language dialect is not recognized\nerror E-CORE-UNSUPPORTED-FEATURE 3:1 feature is not supported by the selected compiler path\n"
        );
        Ok(())
    }

    #[test]
    fn diagnostic_location_failure_is_visible() {
        let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(99));
        let mut rendered = String::new();

        assert_eq!(
            source(b"10 END").and_then(|value| diagnostic
                .render(Some(value), RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-INTERNAL-LOCATION 0:0 diagnostic location could not be resolved\n"
        );
    }

    #[test]
    fn diagnostic_cursor_location_failure_is_visible() {
        let mut cursor = crate::LineCursor::new();
        let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(99));
        let mut rendered = String::new();

        assert_eq!(
            source(b"10 END").and_then(|value| diagnostic
                .render_with_cursor(value, &mut cursor, RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-INTERNAL-LOCATION 0:0 diagnostic location could not be resolved\n"
        );
    }
}
