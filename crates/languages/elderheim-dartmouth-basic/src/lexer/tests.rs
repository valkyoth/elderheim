use super::{
    Basic1BuiltinFunction, Basic1Keyword, Basic1LexError, Basic1LexErrorKind, Basic1TokenKind,
    lex_basic1_statement, lex_basic1_statement_with_limits,
};
use elderheim_core::CompileLimits;

#[derive(Clone, Copy)]
struct ExpectedToken<'source> {
    kind: Basic1TokenKind,
    lexeme: &'source str,
    start: u32,
    end: u32,
}

fn assert_tokens(source: &str, expected: &[ExpectedToken<'_>]) {
    let tokens = lex_basic1_statement(source);
    assert!(tokens.is_ok(), "expected valid tokens: {tokens:?}");

    let tokens = match tokens {
        Ok(tokens) => tokens,
        Err(_) => return,
    };

    assert_eq!(tokens.len(), expected.len());
    for (token, expected) in tokens.iter().zip(expected.iter()) {
        assert_eq!(token.kind, expected.kind);
        assert_eq!(token.lexeme, expected.lexeme);
        assert_eq!(token.span.start(), expected.start);
        assert_eq!(token.span.end(), expected.end);
    }
}

#[test]
fn lexes_print_string_fixture_exactly() {
    assert_tokens(
        "PRINT \"HELLO FROM DARTMOUTH BASIC 1\"",
        &[
            ExpectedToken {
                kind: Basic1TokenKind::Keyword(Basic1Keyword::Print),
                lexeme: "PRINT",
                start: 0,
                end: 5,
            },
            ExpectedToken {
                kind: Basic1TokenKind::StringLiteral,
                lexeme: "\"HELLO FROM DARTMOUTH BASIC 1\"",
                start: 6,
                end: 36,
            },
        ],
    );
}

#[test]
fn lexes_arithmetic_fixture_exactly() {
    assert_tokens(
        "LET X = (7+8)/3",
        &[
            ExpectedToken {
                kind: Basic1TokenKind::Keyword(Basic1Keyword::Let),
                lexeme: "LET",
                start: 0,
                end: 3,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Identifier,
                lexeme: "X",
                start: 4,
                end: 5,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Equal,
                lexeme: "=",
                start: 6,
                end: 7,
            },
            ExpectedToken {
                kind: Basic1TokenKind::LeftParen,
                lexeme: "(",
                start: 8,
                end: 9,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Number,
                lexeme: "7",
                start: 9,
                end: 10,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Plus,
                lexeme: "+",
                start: 10,
                end: 11,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Number,
                lexeme: "8",
                start: 11,
                end: 12,
            },
            ExpectedToken {
                kind: Basic1TokenKind::RightParen,
                lexeme: ")",
                start: 12,
                end: 13,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Slash,
                lexeme: "/",
                start: 13,
                end: 14,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Number,
                lexeme: "3",
                start: 14,
                end: 15,
            },
        ],
    );
}

#[test]
fn lexes_control_flow_and_relations() {
    assert_tokens(
        "IF X < 5 THEN 50",
        &[
            ExpectedToken {
                kind: Basic1TokenKind::Keyword(Basic1Keyword::If),
                lexeme: "IF",
                start: 0,
                end: 2,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Identifier,
                lexeme: "X",
                start: 3,
                end: 4,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Less,
                lexeme: "<",
                start: 5,
                end: 6,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Number,
                lexeme: "5",
                start: 7,
                end: 8,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Keyword(Basic1Keyword::Then),
                lexeme: "THEN",
                start: 9,
                end: 13,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Number,
                lexeme: "50",
                start: 14,
                end: 16,
            },
        ],
    );
}

#[test]
fn lexes_user_and_builtin_functions() {
    assert_tokens(
        "DEF FNS(Z) = SIN(Z)",
        &[
            ExpectedToken {
                kind: Basic1TokenKind::Keyword(Basic1Keyword::Def),
                lexeme: "DEF",
                start: 0,
                end: 3,
            },
            ExpectedToken {
                kind: Basic1TokenKind::UserFunctionIdentifier,
                lexeme: "FNS",
                start: 4,
                end: 7,
            },
            ExpectedToken {
                kind: Basic1TokenKind::LeftParen,
                lexeme: "(",
                start: 7,
                end: 8,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Identifier,
                lexeme: "Z",
                start: 8,
                end: 9,
            },
            ExpectedToken {
                kind: Basic1TokenKind::RightParen,
                lexeme: ")",
                start: 9,
                end: 10,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Equal,
                lexeme: "=",
                start: 11,
                end: 12,
            },
            ExpectedToken {
                kind: Basic1TokenKind::BuiltinFunction(Basic1BuiltinFunction::Sin),
                lexeme: "SIN",
                start: 13,
                end: 16,
            },
            ExpectedToken {
                kind: Basic1TokenKind::LeftParen,
                lexeme: "(",
                start: 16,
                end: 17,
            },
            ExpectedToken {
                kind: Basic1TokenKind::Identifier,
                lexeme: "Z",
                start: 17,
                end: 18,
            },
            ExpectedToken {
                kind: Basic1TokenKind::RightParen,
                lexeme: ")",
                start: 18,
                end: 19,
            },
        ],
    );
}

#[test]
fn lexes_every_committed_basic1_example_statement() {
    for source in [
        include_str!("../../../../../examples/dartmouth-basic-1/hello.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/arithmetic.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/for-next.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/branch.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/def-function.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/read-data.bas"),
    ] {
        let table = crate::parse_basic1_line_table(source);
        assert!(table.is_ok(), "expected valid line table: {table:?}");
        let table = match table {
            Ok(table) => table,
            Err(_) => continue,
        };
        for entry in table.entries() {
            assert_eq!(lex_basic1_statement(entry.statement).map(|_| ()), Ok(()));
        }
    }
}

#[test]
fn rejects_invalid_identifier_shape() {
    assert_eq!(
        lex_basic1_statement("LET ABC = 1").map(|_| ()),
        Err(Basic1LexError {
            kind: Basic1LexErrorKind::InvalidIdentifier,
            span: elderheim_core::Span::checked(4, 7).unwrap_or_else(|_| unreachable_span()),
        })
    );
}

#[test]
fn rejects_unknown_character() {
    assert_eq!(
        lex_basic1_statement("PRINT @").map(|_| ()),
        Err(Basic1LexError {
            kind: Basic1LexErrorKind::UnknownCharacter,
            span: elderheim_core::Span::checked(6, 7).unwrap_or_else(|_| unreachable_span()),
        })
    );
}

#[test]
fn rejects_unterminated_string() {
    assert_eq!(
        lex_basic1_statement("PRINT \"HELLO").map(|_| ()),
        Err(Basic1LexError {
            kind: Basic1LexErrorKind::UnterminatedString,
            span: elderheim_core::Span::checked(6, 12).unwrap_or_else(|_| unreachable_span()),
        })
    );
}

#[test]
fn rejects_sources_over_token_limit() {
    let limits = CompileLimits {
        max_source_bytes: 128,
        max_lines: 10,
        max_tokens: 2,
        max_ir_ops: CompileLimits::DEFAULT.max_ir_ops,
        max_output_bytes: CompileLimits::DEFAULT.max_output_bytes,
    };

    assert_eq!(
        lex_basic1_statement_with_limits("PRINT X, Y", limits).map(|_| ()),
        Err(Basic1LexError {
            kind: Basic1LexErrorKind::TooManyTokens,
            span: elderheim_core::Span::checked(7, 8).unwrap_or_else(|_| unreachable_span()),
        })
    );
}

fn unreachable_span() -> elderheim_core::Span {
    elderheim_core::Span::point(u32::MAX)
}
