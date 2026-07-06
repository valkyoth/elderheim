use crate::Diagnostic;

use super::{PipelineStage, StageOutcome};

pub trait ReportSink {
    fn stage_started(&mut self, stage: PipelineStage);
    fn stage_finished(&mut self, stage: PipelineStage, outcome: StageOutcome);
    fn diagnostic(&mut self, stage: PipelineStage, diagnostic: Diagnostic);
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NullReportSink;

impl ReportSink for NullReportSink {
    fn stage_started(&mut self, _stage: PipelineStage) {}

    fn stage_finished(&mut self, _stage: PipelineStage, _outcome: StageOutcome) {}

    fn diagnostic(&mut self, _stage: PipelineStage, _diagnostic: Diagnostic) {}
}
