#![no_std]

extern crate alloc;

mod corpus;
mod dialect;
mod line_table;

pub use corpus::{Basic1CorpusError, validate_basic1_corpus_source};
pub use dialect::{DartmouthBasicVersion, Dialect};
pub use line_table::{
    Basic1LineNumber, Basic1LineTable, LineTableEntry, LineTableError, LineTableErrorKind,
    parse_basic1_line_table,
};
