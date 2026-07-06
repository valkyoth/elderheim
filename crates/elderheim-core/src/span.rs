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

    pub const fn checked(start: u32, end: u32) -> Result<Self, SpanError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(SpanError::EndBeforeStart)
        }
    }

    #[must_use]
    pub const fn len(self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    #[must_use]
    pub const fn contains_offset(self, offset: u32) -> bool {
        self.start <= offset && offset < self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpanError {
    EndBeforeStart,
}

impl SpanError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::EndBeforeStart => "E-CORE-SPAN-ORDER",
        }
    }

    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::EndBeforeStart => "span end is before span start",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Span, SpanError};

    #[test]
    fn checked_span_rejects_reversed_offsets() {
        assert_eq!(Span::checked(9, 3), Err(SpanError::EndBeforeStart));
    }

    #[test]
    fn span_keeps_offsets_and_length() {
        let span = Span::new(3, 9);
        assert_eq!(span.end, 9);
        assert_eq!(span.len(), 6);
    }

    #[test]
    fn span_contains_offsets_with_exclusive_end() {
        let span = Span::new(3, 9);
        assert!(!span.contains_offset(2));
        assert!(span.contains_offset(3));
        assert!(span.contains_offset(8));
        assert!(!span.contains_offset(9));
    }
}
