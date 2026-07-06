#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub const fn checked(start: u32, end: u32) -> Result<Self, SpanError> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(SpanError::EndBeforeStart)
        }
    }

    pub const fn from_start_len(start: u32, len: u32) -> Result<Self, SpanError> {
        match start.checked_add(len) {
            Some(end) => Self::checked(start, end),
            None => Err(SpanError::EndOverflow),
        }
    }

    #[must_use]
    pub const fn point(offset: u32) -> Self {
        Self {
            start: offset,
            end: offset,
        }
    }

    #[must_use]
    pub const fn start(self) -> u32 {
        self.start
    }

    #[must_use]
    pub const fn end(self) -> u32 {
        self.end
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
    EndOverflow,
}

impl SpanError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::EndBeforeStart => "E-CORE-SPAN-ORDER",
            Self::EndOverflow => "E-CORE-SPAN-OVERFLOW",
        }
    }

    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::EndBeforeStart => "span end is before span start",
            Self::EndOverflow => "span end offset overflows the span model",
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
    fn start_len_span_rejects_overflow() {
        assert_eq!(
            Span::from_start_len(u32::MAX, 1),
            Err(SpanError::EndOverflow)
        );
    }

    #[test]
    fn span_keeps_offsets_and_length() -> Result<(), SpanError> {
        let span = Span::checked(3, 9)?;
        assert_eq!(span.start(), 3);
        assert_eq!(span.end(), 9);
        assert_eq!(span.len(), 6);
        Ok(())
    }

    #[test]
    fn span_contains_offsets_with_exclusive_end() -> Result<(), SpanError> {
        let span = Span::checked(3, 9)?;
        assert!(!span.contains_offset(2));
        assert!(span.contains_offset(3));
        assert!(span.contains_offset(8));
        assert!(!span.contains_offset(9));
        Ok(())
    }

    #[test]
    fn point_span_is_empty_at_offset() {
        let span = Span::point(7);
        assert_eq!(span.start(), 7);
        assert_eq!(span.end(), 7);
        assert!(span.is_empty());
    }
}
