#![no_std]

use elderheim_core::PipelineStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LabelId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirOp {
    Label(LabelId),
    WriteStatic(DataId),
    Exit { code: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LirOp {
    Label(LabelId),
    SysWriteStatic(DataId),
    SysExit { code: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirProgram {
    pub statement_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MirProgram {
    pub op_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LirProgram {
    pub op_count: u32,
}

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
    use super::{DataId, IrBoundary, MirOp};
    use elderheim_core::PipelineStage;

    #[test]
    fn write_static_is_target_neutral() {
        assert_eq!(MirOp::WriteStatic(DataId(7)), MirOp::WriteStatic(DataId(7)));
    }

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
