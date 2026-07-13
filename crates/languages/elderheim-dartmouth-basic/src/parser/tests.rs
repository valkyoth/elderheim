use super::{
    Basic1ParseError, Basic1ParseErrorKind, Basic1ParsedStatement, parse_basic1_program,
    parse_basic1_program_with_limits, parse_hir,
};
use crate::{
    Basic1HirError, Basic1HirLine, Basic1HirProgram, Basic1HirStatement, Basic1HirStatementKind,
    Basic1Keyword, Basic1LexErrorKind, Basic1LineNumber, Basic1Token, Basic1TokenKind,
};
use alloc::vec;
use elderheim_core::{CompileLimits, Span};

fn line_number(value: &str) -> Result<Basic1LineNumber, crate::LineTableErrorKind> {
    Basic1LineNumber::parse(value)
}

#[test]
fn parses_committed_minimal_program_fixtures() -> Result<(), Basic1ParseError> {
    for source in [
        include_str!("../../../../../examples/dartmouth-basic-1/hello.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/print-labels.bas"),
    ] {
        let program = parse_basic1_program(source)?;
        assert!(matches!(
            program.lines.last().map(|line| &line.statement),
            Some(Basic1ParsedStatement::End)
        ));
    }
    Ok(())
}

#[test]
fn parses_blank_and_comma_separated_label_prints() -> Result<(), Basic1ParseError> {
    let program = parse_basic1_program(include_str!(
        "../../../../../examples/dartmouth-basic-1/print-labels.bas"
    ))?;
    let mut lines = program.lines.iter();

    let blank_items = match lines.next().map(|line| &line.statement) {
        Some(Basic1ParsedStatement::Print(print)) => Some(print.items.as_slice()),
        _ => None,
    };
    assert_eq!(blank_items.map(<[_]>::len), Some(0));

    let label_items = match lines.next().map(|line| &line.statement) {
        Some(Basic1ParsedStatement::Print(print)) => Some(print.items.as_slice()),
        _ => None,
    };
    assert_eq!(label_items.map(|items| items.len()), Some(2));
    assert_eq!(
        label_items
            .and_then(|items| items.first())
            .map(|item| item.text),
        Some("LEFT")
    );
    assert_eq!(
        label_items
            .and_then(|items| items.last())
            .map(|item| item.text),
        Some("RIGHT")
    );
    Ok(())
}

#[test]
fn accepts_end_as_the_smallest_program() -> Result<(), Basic1ParseError> {
    let program = parse_basic1_program("10 END\n")?;
    assert_eq!(program.lines.len(), 1);
    assert!(matches!(
        program.lines.first().map(|line| &line.statement),
        Some(Basic1ParsedStatement::End)
    ));
    Ok(())
}

#[test]
fn rejects_malformed_print_separator_sequences() -> Result<(), crate::LineTableErrorKind> {
    for (source, kind, span) in [
        (
            "10 PRINT ,\"A\"\n20 END\n",
            Basic1ParseErrorKind::ExpectedPrintItem,
            Span::checked(6, 7).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?,
        ),
        (
            "10 PRINT \"A\",\n20 END\n",
            Basic1ParseErrorKind::TrailingPrintSeparator,
            Span::checked(9, 10).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?,
        ),
        (
            "10 PRINT \"A\" \"B\"\n20 END\n",
            Basic1ParseErrorKind::ExpectedPrintSeparator,
            Span::checked(10, 13).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?,
        ),
        (
            "10 PRINT \"A\",,\"B\"\n20 END\n",
            Basic1ParseErrorKind::ExpectedPrintItem,
            Span::checked(10, 11).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?,
        ),
    ] {
        assert_eq!(
            parse_basic1_program(source).map(|_| ()),
            Err(Basic1ParseError {
                kind,
                line_number: Some(line_number("10")?),
                span: Some(span),
            })
        );
    }
    Ok(())
}

#[test]
fn rejects_numeric_print_until_expression_parser_stop() -> Result<(), crate::LineTableErrorKind> {
    assert_eq!(
        parse_basic1_program("10 PRINT 1\n20 END\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::UnsupportedPrintItem,
            line_number: Some(line_number("10")?),
            span: Some(
                Span::checked(6, 7).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?
            ),
        })
    );
    Ok(())
}

#[test]
fn rejects_non_minimal_statement_families() -> Result<(), crate::LineTableErrorKind> {
    assert_eq!(
        parse_basic1_program("10 LET X = 1\n20 END\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::UnsupportedStatement,
            line_number: Some(line_number("10")?),
            span: Some(
                Span::checked(0, 3).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?
            ),
        })
    );
    Ok(())
}

#[test]
fn rejects_end_operands_and_statements_after_end() -> Result<(), crate::LineTableErrorKind> {
    assert_eq!(
        parse_basic1_program("10 END 20\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::EndHasOperands,
            line_number: Some(line_number("10")?),
            span: Some(
                Span::checked(4, 6).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?
            ),
        })
    );
    assert_eq!(
        parse_basic1_program("10 END\n20 PRINT \"LATE\"\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::StatementAfterEnd,
            line_number: Some(line_number("20")?),
            span: Some(
                Span::checked(0, 5).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?
            ),
        })
    );
    Ok(())
}

#[test]
fn rejects_program_without_final_end() -> Result<(), crate::LineTableErrorKind> {
    assert_eq!(
        parse_basic1_program("10 PRINT \"NO END\"\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::MissingFinalEnd,
            line_number: Some(line_number("10")?),
            span: Some(Span::point(14)),
        })
    );
    Ok(())
}

#[test]
fn does_not_treat_tokens_as_statement_terminators() -> Result<(), crate::LineTableErrorKind> {
    assert_eq!(
        parse_basic1_program("10 PRINT \"HELLO\" END\n").map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::ExpectedPrintSeparator,
            line_number: Some(line_number("10")?),
            span: Some(
                Span::checked(14, 17).map_err(|_| crate::LineTableErrorKind::InvalidLineNumber)?
            ),
        })
    );
    Ok(())
}

#[test]
fn propagates_frontend_limits_before_parsing() {
    let limits = CompileLimits::with_source_limits(12, 2);
    let error =
        parse_basic1_program_with_limits("10 PRINT \"TOO LARGE\"\n20 END\n", limits).map(|_| ());
    assert!(matches!(
        error,
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::Hir(Basic1HirError::LineTable(_)),
            line_number: None,
            span: None,
        })
    ));
}

#[test]
fn propagates_lexer_errors_before_parsing() {
    let error = parse_basic1_program("10 PRINT @\n20 END\n").map(|_| ());
    assert!(matches!(
        error,
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::Hir(Basic1HirError::Lex(lex_error)),
            line_number: None,
            span: None,
        }) if lex_error.kind == Basic1LexErrorKind::UnknownCharacter
    ));
}

#[test]
fn forged_empty_hir_statements_fail_closed() -> Result<(), crate::LineTableErrorKind> {
    for kind in [Basic1HirStatementKind::Print, Basic1HirStatementKind::End] {
        let program = Basic1HirProgram {
            lines: vec![Basic1HirLine {
                number: line_number("10")?,
                statement: Basic1HirStatement {
                    kind,
                    tokens: vec![],
                    expressions: vec![],
                },
            }],
        };
        assert_eq!(
            parse_hir(program).map(|_| ()),
            Err(Basic1ParseError {
                kind: Basic1ParseErrorKind::EmptyStatement,
                line_number: Some(line_number("10")?),
                span: None,
            })
        );
    }
    Ok(())
}

#[test]
fn forged_string_token_without_quotes_fails_closed() -> Result<(), crate::LineTableErrorKind> {
    let item_span = Span::point(6);
    let program = Basic1HirProgram {
        lines: vec![Basic1HirLine {
            number: line_number("10")?,
            statement: Basic1HirStatement {
                kind: Basic1HirStatementKind::Print,
                tokens: vec![
                    Basic1Token {
                        kind: Basic1TokenKind::Keyword(Basic1Keyword::Print),
                        lexeme: "PRINT",
                        span: Span::point(0),
                    },
                    Basic1Token {
                        kind: Basic1TokenKind::StringLiteral,
                        lexeme: "NOT-QUOTED",
                        span: item_span,
                    },
                ],
                expressions: vec![],
            },
        }],
    };

    assert_eq!(
        parse_hir(program).map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::InvalidStringLiteral,
            line_number: Some(line_number("10")?),
            span: Some(item_span),
        })
    );
    Ok(())
}

#[test]
fn forged_empty_hir_program_reports_missing_end_without_location() {
    let program = Basic1HirProgram { lines: vec![] };
    assert_eq!(
        parse_hir(program).map(|_| ()),
        Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::MissingFinalEnd,
            line_number: None,
            span: None,
        })
    );
}
