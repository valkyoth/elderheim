use super::{NormalizationPolicy, SourceId};
use crate::{CompileLimits, Span};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Source<'a> {
    bytes: &'a [u8],
    line_count: u32,
    id: SourceId,
}

impl<'a> Source<'a> {
    pub fn from_normalized(
        bytes: &'a [u8],
        limits: CompileLimits,
        policy: NormalizationPolicy,
    ) -> Result<Self, SourceError> {
        let line_count = super::normalize::validate_normalized_source(bytes, limits, policy)?;
        let id = super::normalize::source_id_for_normalized(bytes);

        Ok(Self {
            bytes,
            line_count,
            id,
        })
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

    #[must_use]
    pub const fn id(self) -> SourceId {
        self.id
    }

    pub fn line_column(self, offset: u32) -> Result<LineColumn, SourceError> {
        let mut cursor = LineCursor::new();
        self.line_column_from(&mut cursor, offset)
    }

    pub fn line_column_from(
        self,
        cursor: &mut LineCursor,
        offset: u32,
    ) -> Result<LineColumn, SourceError> {
        let offset_usize = usize::try_from(offset).map_err(|_| SourceError::OffsetOutOfBounds)?;
        if offset_usize > self.bytes.len() {
            return Err(SourceError::OffsetOutOfBounds);
        }

        if !cursor.matches_source(self) {
            *cursor = LineCursor::new_for_source(self);
        }

        if offset_usize < cursor.scanned_to {
            *cursor = LineCursor::new_for_source(self);
        }

        for (position, byte) in self.bytes.iter().enumerate().skip(cursor.scanned_to) {
            if position >= offset_usize {
                break;
            }

            if *byte == b'\n' {
                cursor.line = cursor
                    .line
                    .checked_add(1)
                    .ok_or(SourceError::LocationOverflow)?;
                cursor.line_start = position.saturating_add(1);
            }
        }

        cursor.scanned_to = offset_usize;

        let column_usize = offset_usize
            .saturating_sub(cursor.line_start)
            .saturating_add(1);
        let column = u32::try_from(column_usize).map_err(|_| SourceError::LocationOverflow)?;

        Ok(LineColumn {
            line: cursor.line,
            column,
        })
    }

    pub fn span_lines(self, span: Span) -> Result<SourceSpanLines, SourceError> {
        let end = if span.is_empty() {
            span.end()
        } else {
            span.end().saturating_sub(1)
        };

        Ok(SourceSpanLines {
            start: self.line_column(span.start())?,
            end: self.line_column(end)?,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineCursor {
    source_id: Option<SourceId>,
    line: u32,
    line_start: usize,
    scanned_to: usize,
}

impl LineCursor {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_id: None,
            line: 1,
            line_start: 0,
            scanned_to: 0,
        }
    }

    #[must_use]
    pub const fn line_start(self) -> usize {
        self.line_start
    }

    fn new_for_source(source: Source<'_>) -> Self {
        Self {
            source_id: Some(source.id()),
            line: 1,
            line_start: 0,
            scanned_to: 0,
        }
    }

    fn matches_source(self, source: Source<'_>) -> bool {
        self.source_id == Some(source.id())
    }
}

impl Default for LineCursor {
    fn default() -> Self {
        Self::new()
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
    InvalidByte { offset: u32, byte: u8 },
    BlankLine { line: u32 },
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
            Self::InvalidByte { .. } => "E-CORE-SOURCE-BYTE",
            Self::BlankLine { .. } => "E-CORE-SOURCE-BLANK-LINE",
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
            Self::InvalidByte { .. } => "source contains a byte outside the source policy",
            Self::BlankLine { .. } => "source contains a blank line rejected by policy",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LineColumn, LineCursor, Source, SourceError, SourceSpanLines};
    use crate::{CompileLimits, NormalizationPolicy, Span, SpanError};

    fn source(bytes: &[u8]) -> Result<Source<'_>, SourceError> {
        Source::from_normalized(
            bytes,
            CompileLimits::DEFAULT,
            NormalizationPolicy::PRESERVE_BLANK_LINES,
        )
    }

    #[test]
    fn source_counts_lines_without_allocating() {
        assert_eq!(source(b"10 PRINT\n20 END\n").map(Source::line_count), Ok(3));
    }

    #[test]
    fn line_column_lookup_is_one_based() {
        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.line_column(9)),
            Ok(LineColumn { line: 2, column: 1 })
        );
    }

    #[test]
    fn eof_location_is_valid() {
        assert_eq!(
            source(b"10 END").and_then(|value| value.line_column(6)),
            Ok(LineColumn { line: 1, column: 7 })
        );
    }

    #[test]
    fn span_lines_cover_start_and_last_byte() -> Result<(), SpanError> {
        let span = Span::checked(3, 12)?;
        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.span_lines(span)),
            Ok(SourceSpanLines {
                start: LineColumn { line: 1, column: 4 },
                end: LineColumn { line: 2, column: 3 },
            })
        );
        Ok(())
    }

    #[test]
    fn line_cursor_reuses_forward_progress() {
        let mut cursor = LineCursor::new();

        assert_eq!(
            source(b"10 PRINT\n20 GOTO 10\n30 END")
                .and_then(|value| value.line_column_from(&mut cursor, 0)),
            Ok(LineColumn { line: 1, column: 1 })
        );
        assert_eq!(
            source(b"10 PRINT\n20 GOTO 10\n30 END")
                .and_then(|value| value.line_column_from(&mut cursor, 9)),
            Ok(LineColumn { line: 2, column: 1 })
        );
        assert_eq!(
            source(b"10 PRINT\n20 GOTO 10\n30 END")
                .and_then(|value| value.line_column_from(&mut cursor, 20)),
            Ok(LineColumn { line: 3, column: 1 })
        );
    }

    #[test]
    fn line_cursor_resets_for_out_of_order_lookup() {
        let mut cursor = LineCursor::new();

        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.line_column_from(&mut cursor, 9)),
            Ok(LineColumn { line: 2, column: 1 })
        );
        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.line_column_from(&mut cursor, 3)),
            Ok(LineColumn { line: 1, column: 4 })
        );
    }

    #[test]
    fn line_cursor_resets_when_source_changes() {
        let mut cursor = LineCursor::new();

        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.line_column_from(&mut cursor, 9)),
            Ok(LineColumn { line: 2, column: 1 })
        );
        assert_eq!(
            source(b"30 STOP\n40 END").and_then(|value| value.line_column_from(&mut cursor, 0)),
            Ok(LineColumn { line: 1, column: 1 })
        );
    }

    #[test]
    fn line_cursor_resets_when_same_length_source_content_changes() {
        let mut cursor = LineCursor::new();

        assert_eq!(
            source(b"AAAA\nBBBB\nCCCC").and_then(|value| value.line_column_from(&mut cursor, 8)),
            Ok(LineColumn { line: 2, column: 4 })
        );
        assert_eq!(
            source(b"AAAAXBBBBXCCCC").and_then(|value| value.line_column_from(&mut cursor, 10)),
            Ok(LineColumn {
                line: 1,
                column: 11,
            })
        );
    }

    #[test]
    fn line_cursor_exposes_resolved_line_start() {
        let mut cursor = LineCursor::new();

        assert_eq!(
            source(b"10 PRINT\n20 END").and_then(|value| value.line_column_from(&mut cursor, 10)),
            Ok(LineColumn { line: 2, column: 2 })
        );
        assert_eq!(cursor.line_start(), 9);
    }

    #[test]
    fn line_cursor_remains_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<LineCursor>();
    }

    #[test]
    fn source_size_limit_is_enforced() {
        let limits = CompileLimits::with_source_limits(4, 10);
        assert_eq!(
            Source::from_normalized(b"12345", limits, NormalizationPolicy::PRESERVE_BLANK_LINES,),
            Err(SourceError::SourceTooLarge)
        );
    }

    #[test]
    fn source_line_limit_is_enforced() {
        let limits = CompileLimits::with_source_limits(64, 2);
        assert_eq!(
            Source::from_normalized(
                b"1\n2\n3",
                limits,
                NormalizationPolicy::PRESERVE_BLANK_LINES
            ),
            Err(SourceError::TooManyLines)
        );
    }

    #[test]
    fn out_of_bounds_offset_is_rejected() {
        assert_eq!(
            source(b"10 END").and_then(|value| value.line_column(7)),
            Err(SourceError::OffsetOutOfBounds)
        );
    }

    #[test]
    fn public_source_constructor_rejects_raw_control_bytes() {
        assert_eq!(
            Source::from_normalized(
                b"10 PRINT\r20 END",
                CompileLimits::DEFAULT,
                NormalizationPolicy::PRESERVE_BLANK_LINES,
            ),
            Err(SourceError::InvalidByte {
                offset: 8,
                byte: b'\r',
            })
        );
    }

    #[test]
    fn public_source_constructor_enforces_blank_line_policy() {
        assert_eq!(
            Source::from_normalized(
                b"10 PRINT\n\n20 END",
                CompileLimits::DEFAULT,
                NormalizationPolicy::BASIC_STRICT,
            ),
            Err(SourceError::BlankLine { line: 2 })
        );
    }
}
