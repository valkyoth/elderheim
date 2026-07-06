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
    pub const fn is_next(self, next: Self) -> bool {
        match self.ordinal().checked_add(1) {
            Some(expected) => expected == next.ordinal(),
            None => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StageOutcome {
    Complete,
    Failed(Diagnostic),
}
