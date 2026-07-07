#![no_std]

extern crate alloc;

mod corpus;
mod dialect;
mod lexer;
mod line_table;

pub use corpus::{Basic1CorpusError, validate_basic1_corpus_source};
pub use dialect::{DartmouthBasicVersion, Dialect};
pub use lexer::{
    Basic1BuiltinFunction, Basic1Keyword, Basic1LexError, Basic1LexErrorKind, Basic1Token,
    Basic1TokenKind, lex_basic1_statement,
};
pub use line_table::{
    Basic1LineNumber, Basic1LineTable, LineTableEntry, LineTableError, LineTableErrorKind,
    parse_basic1_line_table, parse_basic1_line_table_with_limits,
};
