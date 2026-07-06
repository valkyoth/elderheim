#![no_std]

mod diagnostic;
mod limits;
mod pipeline;
mod report;
mod source;
mod span;

pub use diagnostic::{Diagnostic, DiagnosticCode, DiagnosticDescriptor, RenderStyle, Severity};
pub use limits::CompileLimits;
pub use pipeline::{
    NullReportSink, PipelineError, PipelineStage, ReportSink, StageOutcome, StageStep, run_pipeline,
};
pub use report::{ReportEvent, ReportSection, render_section};
pub use source::{
    BlankLinePolicy, LineColumn, LineCursor, NormalizationPolicy, NormalizedSourceSink, Source,
    SourceError, SourceId, SourceSpanLines, normalize_source,
};
pub use span::{Span, SpanError};

pub type ErrorCode = DiagnosticCode;
