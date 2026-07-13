#![no_std]

extern crate alloc;

mod corpus;
mod dialect;
mod hir;
mod lexer;
mod line_table;
mod parser;

pub use corpus::{Basic1CorpusError, validate_basic1_corpus_source};
pub use dialect::{DartmouthBasicVersion, Dialect};
pub use hir::{
    Basic1HirError, Basic1HirExpression, Basic1HirLine, Basic1HirProgram, Basic1HirStatement,
    Basic1HirStatementKind, build_basic1_hir, build_basic1_hir_with_limits,
    render_basic1_hir_snapshot,
};
pub use lexer::{
    Basic1BuiltinFunction, Basic1Keyword, Basic1LexError, Basic1LexErrorKind, Basic1Token,
    Basic1TokenKind, lex_basic1_statement, lex_basic1_statement_with_limits,
};
pub use line_table::{
    Basic1LineNumber, Basic1LineTable, LineTableEntry, LineTableError, LineTableErrorKind,
    parse_basic1_line_table, parse_basic1_line_table_with_limits,
};
pub use parser::{
    Basic1ParseError, Basic1ParseErrorKind, Basic1ParsedLine, Basic1ParsedProgram,
    Basic1ParsedStatement, Basic1PrintItem, Basic1PrintStatement, parse_basic1_program,
    parse_basic1_program_with_limits,
};
