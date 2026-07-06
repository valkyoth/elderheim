#![no_std]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DartmouthBasicVersion {
    Version1,
    Version2,
    Version4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Dialect {
    pub version: DartmouthBasicVersion,
    pub requires_line_numbers: bool,
}

impl Dialect {
    pub const V1: Self = Self {
        version: DartmouthBasicVersion::Version1,
        requires_line_numbers: true,
    };

    pub const V2: Self = Self {
        version: DartmouthBasicVersion::Version2,
        requires_line_numbers: true,
    };

    pub const V4: Self = Self {
        version: DartmouthBasicVersion::Version4,
        requires_line_numbers: true,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1CorpusError {
    EmptySource,
    EmptyLine,
    MissingLineNumber,
    InvalidLineNumber,
    NonIncreasingLineNumber,
    MissingStatement,
    UnsupportedStatement,
    SessionCommand,
    MissingEnd,
}

pub fn validate_basic1_corpus_source(source: &str) -> Result<(), Basic1CorpusError> {
    if source.is_empty() {
        return Err(Basic1CorpusError::EmptySource);
    }

    let mut previous_number = "";
    let mut last_statement = "";
    let mut saw_line = false;

    for raw_line in source.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            return Err(Basic1CorpusError::EmptyLine);
        }

        let (number, statement) = line
            .split_once(' ')
            .ok_or(Basic1CorpusError::MissingLineNumber)?;
        validate_line_number(number)?;
        if !previous_number.is_empty() && !is_greater_number_text(number, previous_number) {
            return Err(Basic1CorpusError::NonIncreasingLineNumber);
        }

        let statement = statement.trim_start();
        if statement.is_empty() {
            return Err(Basic1CorpusError::MissingStatement);
        }
        validate_basic1_statement(statement)?;

        previous_number = number;
        last_statement = statement;
        saw_line = true;
    }

    if !saw_line {
        return Err(Basic1CorpusError::EmptySource);
    }
    if last_statement != "END" {
        return Err(Basic1CorpusError::MissingEnd);
    }

    Ok(())
}

fn validate_line_number(number: &str) -> Result<(), Basic1CorpusError> {
    if number.is_empty() || number.len() > 5 {
        return Err(Basic1CorpusError::InvalidLineNumber);
    }
    if number.starts_with('0') {
        return Err(Basic1CorpusError::InvalidLineNumber);
    }
    if !number.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(Basic1CorpusError::InvalidLineNumber);
    }
    Ok(())
}

fn is_greater_number_text(current: &str, previous: &str) -> bool {
    match current.len().cmp(&previous.len()) {
        core::cmp::Ordering::Greater => true,
        core::cmp::Ordering::Less => false,
        core::cmp::Ordering::Equal => current > previous,
    }
}

fn validate_basic1_statement(statement: &str) -> Result<(), Basic1CorpusError> {
    if is_session_command(statement) {
        return Err(Basic1CorpusError::SessionCommand);
    }
    if statement == "END"
        || statement.starts_with("LET ")
        || statement.starts_with("PRINT")
        || statement.starts_with("FOR ")
        || statement.starts_with("NEXT ")
        || statement.starts_with("GO TO ")
        || statement.starts_with("GOTO ")
        || statement.starts_with("DEF FN")
        || statement.starts_with("READ ")
        || statement.starts_with("DATA ")
        || is_if_then(statement)
    {
        Ok(())
    } else {
        Err(Basic1CorpusError::UnsupportedStatement)
    }
}

fn is_session_command(statement: &str) -> bool {
    matches!(
        statement,
        "HELLO" | "RUN" | "LIST" | "SAVE" | "STOP" | "READY"
    )
}

fn is_if_then(statement: &str) -> bool {
    statement.starts_with("IF ") && statement.contains(" THEN ")
}

#[cfg(test)]
mod tests {
    use super::{Basic1CorpusError, Dialect, validate_basic1_corpus_source};

    const BASIC1_EXAMPLES: [(&str, &str); 6] = [
        (
            "hello.bas",
            include_str!("../../../../examples/dartmouth-basic-1/hello.bas"),
        ),
        (
            "arithmetic.bas",
            include_str!("../../../../examples/dartmouth-basic-1/arithmetic.bas"),
        ),
        (
            "for-next.bas",
            include_str!("../../../../examples/dartmouth-basic-1/for-next.bas"),
        ),
        (
            "branch.bas",
            include_str!("../../../../examples/dartmouth-basic-1/branch.bas"),
        ),
        (
            "def-function.bas",
            include_str!("../../../../examples/dartmouth-basic-1/def-function.bas"),
        ),
        (
            "read-data.bas",
            include_str!("../../../../examples/dartmouth-basic-1/read-data.bas"),
        ),
    ];

    fn requires_line_numbers(dialect: Dialect) -> bool {
        dialect.requires_line_numbers
    }

    #[test]
    fn dartmouth_versions_require_line_numbers() {
        assert!(requires_line_numbers(Dialect::V1));
        assert!(requires_line_numbers(Dialect::V2));
        assert!(requires_line_numbers(Dialect::V4));
    }

    #[test]
    fn basic1_examples_are_valid_corpus_sources() {
        for (_name, source) in BASIC1_EXAMPLES {
            assert_eq!(validate_basic1_corpus_source(source), Ok(()));
        }
    }

    #[test]
    fn basic1_manifest_lists_all_committed_examples() {
        let manifest = include_str!("../../../../examples/dartmouth-basic-1/manifest.txt");
        for (name, _source) in BASIC1_EXAMPLES {
            assert!(
                manifest.lines().any(|line| line == name),
                "manifest must list {name}"
            );
        }
    }

    #[test]
    fn basic1_corpus_rejects_session_commands() {
        assert_eq!(
            validate_basic1_corpus_source("10 RUN\n20 END\n"),
            Err(Basic1CorpusError::SessionCommand)
        );
    }

    #[test]
    fn basic1_corpus_rejects_unordered_line_numbers() {
        assert_eq!(
            validate_basic1_corpus_source("20 PRINT X\n10 END\n"),
            Err(Basic1CorpusError::NonIncreasingLineNumber)
        );
    }

    #[test]
    fn basic1_corpus_requires_final_end() {
        assert_eq!(
            validate_basic1_corpus_source("10 PRINT X\n20 PRINT X\n"),
            Err(Basic1CorpusError::MissingEnd)
        );
    }
}
