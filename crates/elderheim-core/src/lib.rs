#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[must_use]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    UnsupportedFeature,
    InvalidDialect,
    ProgramTooLarge,
    InvalidExecutableLayout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    pub code: ErrorCode,
    pub severity: Severity,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CompileLimits {
    pub max_source_bytes: usize,
    pub max_lines: usize,
    pub max_tokens: usize,
    pub max_ir_ops: usize,
    pub max_output_bytes: usize,
}

impl CompileLimits {
    pub const DEFAULT: Self = Self {
        max_source_bytes: 1024 * 1024,
        max_lines: 16_384,
        max_tokens: 262_144,
        max_ir_ops: 262_144,
        max_output_bytes: 16 * 1024 * 1024,
    };
}

#[cfg(test)]
mod tests {
    use super::{CompileLimits, Span};

    fn source_fits_in_output(limits: CompileLimits) -> bool {
        limits.max_source_bytes > 0 && limits.max_output_bytes >= limits.max_source_bytes
    }

    #[test]
    fn default_limits_are_bounded() {
        assert!(source_fits_in_output(CompileLimits::DEFAULT));
    }

    #[test]
    fn span_keeps_offsets() {
        assert_eq!(Span::new(3, 9).end, 9);
    }
}
