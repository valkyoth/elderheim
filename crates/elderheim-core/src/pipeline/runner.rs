use crate::Diagnostic;

use super::{PipelineStage, ReportSink, StageOutcome};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PipelineError {
    StageOutOfOrder {
        previous: PipelineStage,
        current: PipelineStage,
    },
    StageMissing {
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
            && previous_stage.ordinal() >= step.stage.ordinal()
        {
            return Err(PipelineError::StageOutOfOrder {
                previous: previous_stage,
                current: step.stage,
            });
        }

        if let Some(previous_stage) = previous
            && !previous_stage.is_next(step.stage)
        {
            return Err(PipelineError::StageMissing {
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

    use super::{PipelineError, ReportSink, StageStep, run_pipeline};
    use crate::{Diagnostic, DiagnosticCode, PipelineStage, Span, StageOutcome};

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

    fn target_stage(context: &mut Context, _sink: &mut dyn ReportSink) -> StageOutcome {
        context.calls.push(PipelineStage::LirToTarget);
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
            StageStep::new(PipelineStage::LirToTarget, target_stage),
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
                PipelineStage::LirToTarget,
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
    fn pipeline_rejects_missing_intermediate_stages() {
        let steps = [
            StageStep::new(PipelineStage::SourceToDiagnostic, source_stage),
            StageStep::new(PipelineStage::LirToTarget, target_stage),
        ];
        let mut context = Context::default();
        let mut sink = Sink::default();

        assert_eq!(
            run_pipeline(&steps, &mut context, &mut sink),
            Err(PipelineError::StageMissing {
                previous: PipelineStage::SourceToDiagnostic,
                current: PipelineStage::LirToTarget,
            })
        );
        assert_eq!(context.calls, [PipelineStage::SourceToDiagnostic]);
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
