use crate::{CompileLimits, Span};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Source<'a> {
    bytes: &'a [u8],
    line_count: u32,
}

impl<'a> Source<'a> {
    pub fn from_bytes(bytes: &'a [u8], limits: CompileLimits) -> Result<Self, SourceError> {
        if bytes.len() > limits.max_source_bytes {
            return Err(SourceError::SourceTooLarge);
        }

        let line_count = count_lines(bytes)?;
        let max_lines = u32::try_from(limits.max_lines).map_err(|_| SourceError::LimitTooLarge)?;

        if line_count > max_lines {
            return Err(SourceError::TooManyLines);
        }

        let max_supported_len =
            usize::try_from(u32::MAX).map_err(|_| SourceError::LimitTooLarge)?;
        if bytes.len() > max_supported_len {
            return Err(SourceError::SourceTooLarge);
        }

        Ok(Self { bytes, line_count })
    }

    #[must_use]
    pub const fn bytes(self) -> &'a [u8] {
        self.bytes
    }

    #[must_use]
    pub const fn len(self) -> usize {
        self.bytes.len()
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }

    #[must_use]
    pub const fn line_count(self) -> u32 {
        self.line_count
    }

    pub fn line_column(self, offset: u32) -> Result<LineColumn, SourceError> {
        let offset_usize = usize::try_from(offset).map_err(|_| SourceError::OffsetOutOfBounds)?;
        if offset_usize > self.bytes.len() {
            return Err(SourceError::OffsetOutOfBounds);
        }

        let mut line = 1_u32;
        let mut line_start = 0_usize;

        for (position, byte) in self.bytes.iter().enumerate() {
            if position >= offset_usize {
                break;
            }

            if *byte == b'\n' {
                line = line.checked_add(1).ok_or(SourceError::LocationOverflow)?;
                line_start = position.saturating_add(1);
            }
        }

        let column_usize = offset_usize.saturating_sub(line_start).saturating_add(1);
        let column = u32::try_from(column_usize).map_err(|_| SourceError::LocationOverflow)?;

        Ok(LineColumn { line, column })
    }

    pub fn span_lines(self, span: Span) -> Result<SourceSpanLines, SourceError> {
        if span.start > span.end {
            return Err(SourceError::InvalidSpan);
        }

        let end = if span.is_empty() {
            span.end
        } else {
            span.end.saturating_sub(1)
        };

        Ok(SourceSpanLines {
            start: self.line_column(span.start)?,
            end: self.line_column(end)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineColumn {
    pub line: u32,
    pub column: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceSpanLines {
    pub start: LineColumn,
    pub end: LineColumn,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceError {
    SourceTooLarge,
    TooManyLines,
    LimitTooLarge,
    OffsetOutOfBounds,
    InvalidSpan,
    LocationOverflow,
}

impl SourceError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SourceTooLarge => "E-CORE-SOURCE-SIZE",
            Self::TooManyLines => "E-CORE-SOURCE-LINES",
            Self::LimitTooLarge => "E-CORE-LIMIT",
            Self::OffsetOutOfBounds => "E-CORE-SOURCE-OFFSET",
            Self::InvalidSpan => "E-CORE-SOURCE-SPAN",
            Self::LocationOverflow => "E-CORE-SOURCE-LOCATION",
        }
    }

    #[must_use]
    pub const fn message(self) -> &'static str {
        match self {
            Self::SourceTooLarge => "source exceeds configured byte limit",
            Self::TooManyLines => "source exceeds configured line limit",
            Self::LimitTooLarge => "compile limit is too large for this operation",
            Self::OffsetOutOfBounds => "source offset is outside the source byte range",
            Self::InvalidSpan => "source span end is before source span start",
            Self::LocationOverflow => "source location does not fit in the location model",
        }
    }
}

fn count_lines(bytes: &[u8]) -> Result<u32, SourceError> {
    if bytes.is_empty() {
        return Ok(0);
    }

    let mut lines = 1_u32;
    for byte in bytes {
        if *byte == b'\n' {
            lines = lines.checked_add(1).ok_or(SourceError::LocationOverflow)?;
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::{LineColumn, Source, SourceError, SourceSpanLines};
    use crate::{CompileLimits, Span};

    #[test]
    fn source_counts_lines_without_allocating() {
        let source = Source::from_bytes(b"10 PRINT\n20 END\n", CompileLimits::DEFAULT);
        assert_eq!(source.map(Source::line_count), Ok(3));
    }

    #[test]
    fn line_column_lookup_is_one_based() {
        let source = Source::from_bytes(b"10 PRINT\n20 END", CompileLimits::DEFAULT);
        assert_eq!(
            source.and_then(|value| value.line_column(9)),
            Ok(LineColumn { line: 2, column: 1 })
        );
    }

    #[test]
    fn eof_location_is_valid() {
        let source = Source::from_bytes(b"10 END", CompileLimits::DEFAULT);
        assert_eq!(
            source.and_then(|value| value.line_column(6)),
            Ok(LineColumn { line: 1, column: 7 })
        );
    }

    #[test]
    fn span_lines_cover_start_and_last_byte() {
        let source = Source::from_bytes(b"10 PRINT\n20 END", CompileLimits::DEFAULT);
        assert_eq!(
            source.and_then(|value| value.span_lines(Span::new(3, 12))),
            Ok(SourceSpanLines {
                start: LineColumn { line: 1, column: 4 },
                end: LineColumn { line: 2, column: 3 },
            })
        );
    }

    #[test]
    fn source_size_limit_is_enforced() {
        let limits = CompileLimits::with_source_limits(4, 10);
        assert_eq!(
            Source::from_bytes(b"12345", limits),
            Err(SourceError::SourceTooLarge)
        );
    }

    #[test]
    fn source_line_limit_is_enforced() {
        let limits = CompileLimits::with_source_limits(64, 2);
        assert_eq!(
            Source::from_bytes(b"1\n2\n3", limits),
            Err(SourceError::TooManyLines)
        );
    }

    #[test]
    fn out_of_bounds_offset_is_rejected() {
        let source = Source::from_bytes(b"10 END", CompileLimits::DEFAULT);
        assert_eq!(
            source.and_then(|value| value.line_column(7)),
            Err(SourceError::OffsetOutOfBounds)
        );
    }
}
