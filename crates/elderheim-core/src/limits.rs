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

    #[must_use]
    pub const fn with_source_limits(max_source_bytes: usize, max_lines: usize) -> Self {
        Self {
            max_source_bytes,
            max_lines,
            max_tokens: Self::DEFAULT.max_tokens,
            max_ir_ops: Self::DEFAULT.max_ir_ops,
            max_output_bytes: Self::DEFAULT.max_output_bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CompileLimits;

    fn source_fits_in_output(limits: CompileLimits) -> bool {
        limits.max_source_bytes > 0 && limits.max_output_bytes >= limits.max_source_bytes
    }

    #[test]
    fn default_limits_are_bounded() {
        assert!(source_fits_in_output(CompileLimits::DEFAULT));
    }

    #[test]
    fn source_limit_constructor_keeps_other_defaults() {
        let limits = CompileLimits::with_source_limits(12, 3);
        assert_eq!(limits.max_source_bytes, 12);
        assert_eq!(limits.max_lines, 3);
        assert_eq!(limits.max_tokens, CompileLimits::DEFAULT.max_tokens);
    }
}
