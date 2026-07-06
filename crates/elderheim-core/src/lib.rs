#![no_std]

mod diagnostic;
mod limits;
mod source;
mod span;

pub use diagnostic::{Diagnostic, DiagnosticCode, RenderStyle, Severity};
pub use limits::CompileLimits;
pub use source::{LineColumn, LineCursor, Source, SourceError, SourceSpanLines};
pub use span::{Span, SpanError};

pub type ErrorCode = DiagnosticCode;
