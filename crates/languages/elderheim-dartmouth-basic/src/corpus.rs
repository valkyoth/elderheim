use crate::line_table::{Basic1LineTable, LineTableErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1CorpusError {
    EmptySource,
    SourceTooLarge,
    TooManyLines,
    EmptyLine,
    MissingLineNumber,
    InvalidLineNumber,
    MissingStatementSeparator,
    DuplicateLineNumber,
    NonIncreasingLineNumber,
    MissingStatement,
    UnsupportedStatement,
    SessionCommand,
    MissingEnd,
}

pub fn validate_basic1_corpus_source(source: &str) -> Result<(), Basic1CorpusError> {
    let table = Basic1LineTable::parse(source).map_err(map_line_table_error)?;

    for entry in table.entries() {
        validate_basic1_statement(entry.statement)?;
    }

    let last_statement = table
        .last()
        .map(|entry| entry.statement)
        .ok_or(Basic1CorpusError::EmptySource)?;
    if last_statement != "END" {
        return Err(Basic1CorpusError::MissingEnd);
    }

    Ok(())
}

fn map_line_table_error(error: crate::line_table::LineTableError) -> Basic1CorpusError {
    match error.kind {
        LineTableErrorKind::EmptySource => Basic1CorpusError::EmptySource,
        LineTableErrorKind::SourceTooLarge => Basic1CorpusError::SourceTooLarge,
        LineTableErrorKind::TooManyLines => Basic1CorpusError::TooManyLines,
        LineTableErrorKind::EmptyPhysicalLine => Basic1CorpusError::EmptyLine,
        LineTableErrorKind::MissingLineNumber => Basic1CorpusError::MissingLineNumber,
        LineTableErrorKind::InvalidLineNumber => Basic1CorpusError::InvalidLineNumber,
        LineTableErrorKind::MissingStatementSeparator => {
            Basic1CorpusError::MissingStatementSeparator
        }
        LineTableErrorKind::EmptyNumberedLine => Basic1CorpusError::MissingStatement,
        LineTableErrorKind::DuplicateLineNumber => Basic1CorpusError::DuplicateLineNumber,
        LineTableErrorKind::OutOfOrderLineNumber => Basic1CorpusError::NonIncreasingLineNumber,
    }
}

fn validate_basic1_statement(statement: &str) -> Result<(), Basic1CorpusError> {
    if is_session_command(statement) {
        return Err(Basic1CorpusError::SessionCommand);
    }
    if statement == "END"
        || statement.starts_with("LET ")
        || starts_with_keyword(statement, "PRINT")
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

fn starts_with_keyword(statement: &str, keyword: &str) -> bool {
    match statement.strip_prefix(keyword) {
        Some(rest) => rest.is_empty() || rest.starts_with(' '),
        None => false,
    }
}

fn is_if_then(statement: &str) -> bool {
    statement.starts_with("IF ") && statement.contains(" THEN ")
}

#[cfg(test)]
mod tests {
    use super::{Basic1CorpusError, validate_basic1_corpus_source};
    use crate::parse_basic1_line_table;

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

    #[test]
    fn basic1_examples_are_valid_corpus_sources() {
        for (_name, source) in BASIC1_EXAMPLES {
            assert_eq!(validate_basic1_corpus_source(source), Ok(()));
        }
    }

    #[test]
    fn basic1_examples_have_valid_line_tables() {
        for (_name, source) in BASIC1_EXAMPLES {
            assert!(parse_basic1_line_table(source).is_ok());
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
    fn basic1_corpus_rejects_print_prefix_confusion() {
        assert_eq!(
            validate_basic1_corpus_source("10 PRINTED X\n20 END\n"),
            Err(Basic1CorpusError::UnsupportedStatement)
        );
        assert_eq!(
            validate_basic1_corpus_source("10 PRINTER\n20 END\n"),
            Err(Basic1CorpusError::UnsupportedStatement)
        );
    }

    #[test]
    fn basic1_corpus_rejects_duplicate_line_numbers() {
        assert_eq!(
            validate_basic1_corpus_source("10 PRINT X\n10 END\n"),
            Err(Basic1CorpusError::DuplicateLineNumber)
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
