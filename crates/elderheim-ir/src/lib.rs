#![no_std]

mod error;
mod hir;
mod ids;
mod lir;
mod lowering;
mod mir;

pub use error::{IrError, IrLayer};
pub use hir::{HirNode, HirNodeKind, HirProgram, ValidatedHir, validate_hir};
pub use ids::{DataId, HirNodeId, HirSymbolId, LirLabelId, LirSymbolId, MirLabelId, MirValueId};
pub use lir::{LirOp, LirProgram, MAX_LIR_OPS, ValidatedLir, validate_lir};
pub use lowering::{HirToMirLowerer, LirSink, MirSink, MirToLirLowerer};
pub use mir::{MAX_MIR_OPS, MirOp, MirProgram, ValidatedMir, validate_mir};

use elderheim_core::PipelineStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TargetProgram {
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrBoundary {
    HirToMir,
    MirToLir,
    LirToTarget,
}

impl IrBoundary {
    #[must_use]
    pub const fn pipeline_stage(self) -> PipelineStage {
        match self {
            Self::HirToMir => PipelineStage::HirToMir,
            Self::MirToLir => PipelineStage::MirToLir,
            Self::LirToTarget => PipelineStage::LirToTarget,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IrBoundary;
    use elderheim_core::PipelineStage;

    #[test]
    fn ir_boundaries_map_to_pipeline_stages() {
        assert_eq!(
            IrBoundary::HirToMir.pipeline_stage(),
            PipelineStage::HirToMir
        );
        assert_eq!(
            IrBoundary::MirToLir.pipeline_stage(),
            PipelineStage::MirToLir
        );
        assert_eq!(
            IrBoundary::LirToTarget.pipeline_stage(),
            PipelineStage::LirToTarget
        );
    }
}
