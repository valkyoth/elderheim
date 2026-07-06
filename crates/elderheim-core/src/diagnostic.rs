use core::fmt::{self, Write};

use crate::{LineColumn, Source, SourceError, Span};

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
        }
    }
}

fn render_compact<W: Write>(
    diagnostic: Diagnostic,
    source: Option<Source<'_>>,
    writer: &mut W,
) -> fmt::Result {
    let location = source
        .and_then(|value| value.line_column(diagnostic.span.start).ok())
        .unwrap_or(LineColumn { line: 0, column: 0 });

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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::String;

    use super::{Diagnostic, DiagnosticCode, RenderStyle, Severity};
    use crate::{CompileLimits, Source, SourceError, Span};

    #[test]
    fn source_errors_map_to_stable_diagnostic_codes() {
        assert_eq!(
            DiagnosticCode::from(SourceError::TooManyLines).code(),
            "E-CORE-SOURCE-LINES"
        );
    }

    #[test]
    fn diagnostic_compact_rendering_is_golden() {
        let source = Source::from_bytes(b"10 PRINT\n20 END", CompileLimits::DEFAULT);
        let diagnostic = Diagnostic::new(
            DiagnosticCode::UnsupportedFeature,
            Severity::Error,
            Span::new(9, 15),
        );
        let mut rendered = String::new();
        assert_eq!(
            source.and_then(|value| diagnostic
                .render(Some(value), RenderStyle::Compact, &mut rendered)
                .map_err(|_| SourceError::LocationOverflow)),
            Ok(())
        );
        assert_eq!(
            rendered,
            "error E-CORE-UNSUPPORTED-FEATURE 2:1 feature is not supported by the selected compiler path\n"
        );
    }

    #[test]
    fn diagnostic_without_source_uses_zero_location() {
        let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::new(0, 0));
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
}
