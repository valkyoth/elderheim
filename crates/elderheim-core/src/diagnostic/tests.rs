extern crate std;

use std::string::String;

use super::{Diagnostic, DiagnosticCode, DiagnosticDescriptor, RenderStyle, Severity};
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
fn diagnostic_code_registry_is_stable() {
    let mut registry = DiagnosticCode::ALL.iter();
    assert_eq!(DiagnosticCode::ALL.len(), DiagnosticCode::variant_count());
    assert_eq!(
        registry.next().copied().map(DiagnosticCode::descriptor),
        Some(DiagnosticDescriptor {
            code: DiagnosticCode::UnsupportedFeature,
            identifier: "E-CORE-UNSUPPORTED-FEATURE",
            message: "feature is not supported by the selected compiler path",
            default_severity: Severity::Error,
        })
    );
    assert_eq!(
        registry.last().copied().map(DiagnosticCode::descriptor),
        Some(DiagnosticDescriptor {
            code: DiagnosticCode::InternalLocation,
            identifier: "E-CORE-INTERNAL-LOCATION",
            message: "diagnostic location could not be resolved",
            default_severity: Severity::Error,
        })
    );

    for (left_index, left) in DiagnosticCode::ALL.iter().enumerate() {
        for (right_index, right) in DiagnosticCode::ALL.iter().enumerate() {
            if left_index != right_index {
                assert_ne!(left, right);
            }
        }
    }
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
fn diagnostic_snippet_rendering_is_golden() -> Result<(), SpanError> {
    let diagnostic = Diagnostic::error(DiagnosticCode::UnsupportedFeature, Span::checked(12, 15)?);
    let mut rendered = String::new();

    assert_eq!(
        source(b"10 PRINT\n20 GOTO 10\n30 END").and_then(|value| diagnostic
            .render(Some(value), RenderStyle::Snippet, &mut rendered)
            .map_err(|_| SourceError::LocationOverflow)),
        Ok(())
    );
    assert_eq!(
        rendered,
        "error E-CORE-UNSUPPORTED-FEATURE 2:4 feature is not supported by the selected compiler path\n --> 2:4\n  |\n2 | 20 GOTO 10\n  |    ^\n"
    );
    Ok(())
}

#[test]
fn diagnostic_snippet_without_source_uses_compact_zero_location() {
    let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(0));
    let mut rendered = String::new();

    assert_eq!(
        diagnostic.render(None, RenderStyle::Snippet, &mut rendered),
        Ok(())
    );
    assert_eq!(
        rendered,
        "error E-CORE-INVALID-DIALECT 0:0 selected language dialect is not recognized\n"
    );
}

#[test]
fn diagnostic_snippet_location_failure_is_visible() {
    let diagnostic = Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(99));
    let mut rendered = String::new();

    assert_eq!(
        source(b"10 END").and_then(|value| diagnostic
            .render(Some(value), RenderStyle::Snippet, &mut rendered)
            .map_err(|_| SourceError::LocationOverflow)),
        Ok(())
    );
    assert_eq!(
        rendered,
        "error E-CORE-INTERNAL-LOCATION 0:0 diagnostic location could not be resolved\n"
    );
}

#[test]
fn source_bound_diagnostic_rejects_wrong_source() {
    let expected_source = source(b"10 END");
    let wrong_source = source(b"20 END");
    let diagnostic = expected_source.map(Source::id).map(|id| {
        Diagnostic::error(DiagnosticCode::InvalidDialect, Span::point(0)).with_source_id(id)
    });
    let mut rendered = String::new();

    assert_eq!(
        diagnostic.and_then(|value| wrong_source.and_then(|source| value
            .render(Some(source), RenderStyle::Snippet, &mut rendered)
            .map_err(|_| SourceError::LocationOverflow))),
        Ok(())
    );
    assert_eq!(
        rendered,
        "error E-CORE-INTERNAL-LOCATION 0:0 diagnostic location could not be resolved\n"
    );
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
