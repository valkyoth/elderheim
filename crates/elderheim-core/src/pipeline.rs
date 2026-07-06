use crate::Diagnostic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PipelineStage {
    SourceToDiagnostic,
    HirToMir,
    MirToLir,
    LirToTarget,
}

impl PipelineStage {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::SourceToDiagnostic => "source-to-diagnostic",
            Self::HirToMir => "hir-to-mir",
            Self::MirToLir => "mir-to-lir",
            Self::LirToTarget => "lir-to-target",
        }
    }

    #[must_use]
    pub const fn ordinal(self) -> u8 {
        match self {
            Self::SourceToDiagnostic => 0,
            Self::HirToMir => 1,
            Self::MirToLir => 2,
            Self::LirToTarget => 3,
        }
    }

    #[must_use]
    pub const fn is_before(self, next: Self) -> bool {
        self.ordinal() < next.ordinal()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageOutcome {
    Complete,
    Failed(Diagnostic),
}

#[derive(Clone, Copy)]
pub struct StageStep<C> {
    pub stage: PipelineStage,
    pub run: fn(&mut C, &mut dyn ReportSink) -> StageOutcome,
}

impl<C> StageStep<C> {
    #[must_use]
    pub const fn new(
        stage: PipelineStage,
        run: fn(&mut C, &mut dyn ReportSink) -> StageOutcome,
    ) -> Self {
        Self { stage, run }
    }
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PipelineError {
    StageOutOfOrder {
        previous: PipelineStage,
        current: PipelineStage,
    },
    StageFailed {
        stage: PipelineStage,
        diagnostic: Diagnostic,
    },
}

pub fn run_pipeline<C>(
    steps: &[StageStep<C>],
    context: &mut C,
    sink: &mut dyn ReportSink,
) -> Result<(), PipelineError> {
    let mut previous: Option<PipelineStage> = None;

    for step in steps {
        if let Some(previous_stage) = previous
            && !previous_stage.is_before(step.stage)
        {
            return Err(PipelineError::StageOutOfOrder {
                previous: previous_stage,
                current: step.stage,
            });
        }

        sink.stage_started(step.stage);
        let outcome = (step.run)(context, sink);

        if let StageOutcome::Failed(diagnostic) = outcome {
            sink.diagnostic(step.stage, diagnostic);
            sink.stage_finished(step.stage, outcome);
            return Err(PipelineError::StageFailed {
                stage: step.stage,
                diagnostic,
            });
        }

        sink.stage_finished(step.stage, outcome);
        previous = Some(step.stage);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::vec::Vec;

    use super::{PipelineError, PipelineStage, ReportSink, StageOutcome, StageStep, run_pipeline};
    use crate::{Diagnostic, DiagnosticCode, Span};

    #[derive(Default)]
    struct Context {
        calls: Vec<PipelineStage>,
        fail_hir: bool,
    }

    #[derive(Default)]
    struct Sink {
        started: Vec<PipelineStage>,
        finished: Vec<PipelineStage>,
        diagnostics: Vec<PipelineStage>,
    }

    impl ReportSink for Sink {
        fn stage_started(&mut self, stage: PipelineStage) {
            self.started.push(stage);
        }

        fn stage_finished(&mut self, stage: PipelineStage, _outcome: StageOutcome) {
            self.finished.push(stage);
        }

        fn diagnostic(&mut self, stage: PipelineStage, _diagnostic: Diagnostic) {
            self.diagnostics.push(stage);
        }
    }

    fn source_stage(context: &mut Context, _sink: &mut dyn ReportSink) -> StageOutcome {
        context.calls.push(PipelineStage::SourceToDiagnostic);
        StageOutcome::Complete
    }

    fn hir_stage(context: &mut Context, _sink: &mut dyn ReportSink) -> StageOutcome {
        context.calls.push(PipelineStage::HirToMir);
        if context.fail_hir {
            StageOutcome::Failed(Diagnostic::error(
                DiagnosticCode::UnsupportedFeature,
                Span::point(0),
            ))
        } else {
            StageOutcome::Complete
        }
    }

    fn mir_stage(context: &mut Context, _sink: &mut dyn ReportSink) -> StageOutcome {
        context.calls.push(PipelineStage::MirToLir);
        StageOutcome::Complete
    }

    #[test]
    fn empty_pipeline_passes() {
        let mut context = Context::default();
        let mut sink = Sink::default();
        assert_eq!(
            run_pipeline::<Context>(&[], &mut context, &mut sink),
            Ok(())
        );
        assert!(context.calls.is_empty());
        assert!(sink.started.is_empty());
    }

    #[test]
    fn pipeline_runs_stages_in_order() {
        let steps = [
            StageStep::new(PipelineStage::SourceToDiagnostic, source_stage),
            StageStep::new(PipelineStage::HirToMir, hir_stage),
            StageStep::new(PipelineStage::MirToLir, mir_stage),
        ];
        let mut context = Context::default();
        let mut sink = Sink::default();

        assert_eq!(run_pipeline(&steps, &mut context, &mut sink), Ok(()));
        assert_eq!(
            context.calls,
            [
                PipelineStage::SourceToDiagnostic,
                PipelineStage::HirToMir,
                PipelineStage::MirToLir,
            ]
        );
        assert_eq!(sink.started, context.calls);
        assert_eq!(sink.finished, context.calls);
    }

    #[test]
    fn pipeline_rejects_out_of_order_stages() {
        let steps = [
            StageStep::new(PipelineStage::HirToMir, hir_stage),
            StageStep::new(PipelineStage::SourceToDiagnostic, source_stage),
        ];
        let mut context = Context::default();
        let mut sink = Sink::default();

        assert_eq!(
            run_pipeline(&steps, &mut context, &mut sink),
            Err(PipelineError::StageOutOfOrder {
                previous: PipelineStage::HirToMir,
                current: PipelineStage::SourceToDiagnostic,
            })
        );
        assert_eq!(context.calls, [PipelineStage::HirToMir]);
    }

    #[test]
    fn pipeline_stops_on_stage_error() {
        let steps = [
            StageStep::new(PipelineStage::SourceToDiagnostic, source_stage),
            StageStep::new(PipelineStage::HirToMir, hir_stage),
            StageStep::new(PipelineStage::MirToLir, mir_stage),
        ];
        let mut context = Context {
            calls: Vec::new(),
            fail_hir: true,
        };
        let mut sink = Sink::default();

        let result = run_pipeline(&steps, &mut context, &mut sink);
        assert!(matches!(
            result,
            Err(PipelineError::StageFailed {
                stage: PipelineStage::HirToMir,
                ..
            })
        ));
        assert_eq!(
            context.calls,
            [PipelineStage::SourceToDiagnostic, PipelineStage::HirToMir]
        );
        assert_eq!(sink.diagnostics, [PipelineStage::HirToMir]);
    }
}
