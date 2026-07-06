use alloc::{collections::BTreeSet, vec::Vec};

use elderheim_core::CompileLimits;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Basic1LineNumber(u32);

impl Basic1LineNumber {
    pub const MAX: u32 = 99_999;
    pub const MAX_DIGITS: usize = 5;

    pub fn parse(text: &str) -> Result<Self, LineTableErrorKind> {
        if text.is_empty() || text.len() > Self::MAX_DIGITS || text.starts_with('0') {
            return Err(LineTableErrorKind::InvalidLineNumber);
        }

        let mut value = 0_u32;
        for byte in text.bytes() {
            let digit = match byte {
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                b'3' => 3,
                b'4' => 4,
                b'5' => 5,
                b'6' => 6,
                b'7' => 7,
                b'8' => 8,
                b'9' => 9,
                _ => return Err(LineTableErrorKind::InvalidLineNumber),
            };
            value = value
                .checked_mul(10)
                .and_then(|scaled| scaled.checked_add(digit))
                .ok_or(LineTableErrorKind::InvalidLineNumber)?;
        }

        if value > Self::MAX {
            return Err(LineTableErrorKind::InvalidLineNumber);
        }

        Ok(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineTableErrorKind {
    EmptySource,
    SourceTooLarge,
    TooManyLines,
    EmptyPhysicalLine,
    MissingLineNumber,
    InvalidLineNumber,
    MissingStatementSeparator,
    EmptyNumberedLine,
    DuplicateLineNumber,
    OutOfOrderLineNumber,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineTableError {
    pub kind: LineTableErrorKind,
    pub physical_line_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LineTableEntry<'source> {
    pub number: Basic1LineNumber,
    pub statement: &'source str,
    pub physical_line_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1LineTable<'source> {
    entries: Vec<LineTableEntry<'source>>,
}

impl<'source> Basic1LineTable<'source> {
    pub fn parse(source: &'source str) -> Result<Self, LineTableError> {
        parse_basic1_line_table(source)
    }

    pub fn parse_with_limits(
        source: &'source str,
        limits: CompileLimits,
    ) -> Result<Self, LineTableError> {
        parse_basic1_line_table_with_limits(source, limits)
    }

    pub fn entries(&self) -> &[LineTableEntry<'source>] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn last(&self) -> Option<&LineTableEntry<'source>> {
        self.entries.last()
    }
}

pub fn parse_basic1_line_table(source: &str) -> Result<Basic1LineTable<'_>, LineTableError> {
    parse_basic1_line_table_with_limits(source, CompileLimits::DEFAULT)
}

pub fn parse_basic1_line_table_with_limits(
    source: &str,
    limits: CompileLimits,
) -> Result<Basic1LineTable<'_>, LineTableError> {
    if source.is_empty() {
        return Err(error(LineTableErrorKind::EmptySource, 0));
    }
    if source.len() > limits.max_source_bytes {
        return Err(error(LineTableErrorKind::SourceTooLarge, 0));
    }

    let mut entries = Vec::new();
    let mut seen_numbers = BTreeSet::new();
    let mut previous_number = None;

    for (physical_line_index, raw_line) in source.lines().enumerate() {
        if physical_line_index >= limits.max_lines {
            return Err(error(LineTableErrorKind::TooManyLines, physical_line_index));
        }

        let line = raw_line.trim_end();
        if line.is_empty() {
            return Err(error(
                LineTableErrorKind::EmptyPhysicalLine,
                physical_line_index,
            ));
        }

        let (number, statement) = parse_line(line, physical_line_index)?;
        if !seen_numbers.insert(number.get()) {
            return Err(error(
                LineTableErrorKind::DuplicateLineNumber,
                physical_line_index,
            ));
        }
        if previous_number.is_some_and(|previous| number <= previous) {
            return Err(error(
                LineTableErrorKind::OutOfOrderLineNumber,
                physical_line_index,
            ));
        }

        entries.push(LineTableEntry {
            number,
            statement,
            physical_line_index,
        });
        previous_number = Some(number);
    }

    if entries.is_empty() {
        return Err(error(LineTableErrorKind::EmptySource, 0));
    }

    Ok(Basic1LineTable { entries })
}

fn parse_line(
    line: &str,
    physical_line_index: usize,
) -> Result<(Basic1LineNumber, &str), LineTableError> {
    let digit_count = count_leading_digits(line);
    if digit_count == 0 {
        return Err(error(
            LineTableErrorKind::MissingLineNumber,
            physical_line_index,
        ));
    }

    let (number_text, rest) = line.split_at(digit_count);
    let number =
        Basic1LineNumber::parse(number_text).map_err(|kind| error(kind, physical_line_index))?;

    if rest.is_empty() {
        return Err(error(
            LineTableErrorKind::EmptyNumberedLine,
            physical_line_index,
        ));
    }
    if !rest.starts_with(' ') && !rest.starts_with('\t') {
        return Err(error(
            LineTableErrorKind::MissingStatementSeparator,
            physical_line_index,
        ));
    }

    let statement = rest.trim_start_matches([' ', '\t']);
    if statement.is_empty() {
        return Err(error(
            LineTableErrorKind::EmptyNumberedLine,
            physical_line_index,
        ));
    }

    Ok((number, statement))
}

fn count_leading_digits(line: &str) -> usize {
    let mut count = 0;
    for byte in line.bytes() {
        if !byte.is_ascii_digit() {
            return count;
        }
        count = count.saturating_add(1);
    }
    count
}

const fn error(kind: LineTableErrorKind, physical_line_index: usize) -> LineTableError {
    LineTableError {
        kind,
        physical_line_index,
    }
}

#[cfg(test)]
mod tests {
    use super::{Basic1LineNumber, LineTableError, LineTableErrorKind, parse_basic1_line_table};
    use crate::parse_basic1_line_table_with_limits;
    use elderheim_core::CompileLimits;

    #[test]
    fn parses_increasing_numbered_lines() {
        let table = parse_basic1_line_table("10 PRINT 1\n20 END\n");
        assert!(table.is_ok(), "expected valid line table: {table:?}");

        let table = match table {
            Ok(table) => table,
            Err(_) => return,
        };
        let mut entries = table.entries().iter();
        let first = entries.next();
        let second = entries.next();

        assert_eq!(table.len(), 2);
        assert_eq!(first.map(|entry| entry.number.get()), Some(10));
        assert_eq!(first.map(|entry| entry.statement), Some("PRINT 1"));
        assert_eq!(second.map(|entry| entry.number.get()), Some(20));
        assert_eq!(second.map(|entry| entry.statement), Some("END"));
    }

    #[test]
    fn line_number_parser_rejects_malformed_numbers() {
        assert_eq!(
            Basic1LineNumber::parse(""),
            Err(LineTableErrorKind::InvalidLineNumber)
        );
        assert_eq!(
            Basic1LineNumber::parse("01"),
            Err(LineTableErrorKind::InvalidLineNumber)
        );
        assert_eq!(
            Basic1LineNumber::parse("100000"),
            Err(LineTableErrorKind::InvalidLineNumber)
        );
        assert_eq!(
            Basic1LineNumber::parse("10A"),
            Err(LineTableErrorKind::InvalidLineNumber)
        );
    }

    #[test]
    fn rejects_duplicate_line_numbers_before_order_policy() {
        assert_eq!(
            parse_basic1_line_table("10 PRINT 1\n20 PRINT 2\n10 END\n"),
            Err(LineTableError {
                kind: LineTableErrorKind::DuplicateLineNumber,
                physical_line_index: 2,
            })
        );
    }

    #[test]
    fn rejects_out_of_order_line_numbers() {
        assert_eq!(
            parse_basic1_line_table("20 PRINT 1\n10 END\n"),
            Err(LineTableError {
                kind: LineTableErrorKind::OutOfOrderLineNumber,
                physical_line_index: 1,
            })
        );
    }

    #[test]
    fn rejects_empty_numbered_lines() {
        assert_eq!(
            parse_basic1_line_table("10 PRINT 1\n20   \n30 END\n"),
            Err(LineTableError {
                kind: LineTableErrorKind::EmptyNumberedLine,
                physical_line_index: 1,
            })
        );
    }

    #[test]
    fn rejects_missing_statement_separator() {
        assert_eq!(
            parse_basic1_line_table("10PRINT 1\n20 END\n"),
            Err(LineTableError {
                kind: LineTableErrorKind::MissingStatementSeparator,
                physical_line_index: 0,
            })
        );
    }

    #[test]
    fn rejects_sources_over_compile_byte_limit() {
        assert_eq!(
            parse_basic1_line_table_with_limits(
                "10 PRINT 1\n20 END\n",
                CompileLimits::with_source_limits(8, 10),
            ),
            Err(LineTableError {
                kind: LineTableErrorKind::SourceTooLarge,
                physical_line_index: 0,
            })
        );
    }

    #[test]
    fn rejects_sources_over_compile_line_limit() {
        assert_eq!(
            parse_basic1_line_table_with_limits(
                "10 PRINT 1\n20 PRINT 2\n30 END\n",
                CompileLimits::with_source_limits(128, 2),
            ),
            Err(LineTableError {
                kind: LineTableErrorKind::TooManyLines,
                physical_line_index: 2,
            })
        );
    }
}
