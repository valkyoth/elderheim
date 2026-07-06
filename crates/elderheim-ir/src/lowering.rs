use crate::{IrError, LirOp, MirOp, ValidatedHir, ValidatedMir};

pub trait MirSink {
    fn push_mir(&mut self, op: MirOp) -> Result<(), IrError>;
}

pub trait LirSink {
    fn push_lir(&mut self, op: LirOp) -> Result<(), IrError>;
}

pub trait HirToMirLowerer {
    fn lower_hir_to_mir(
        &self,
        program: ValidatedHir<'_>,
        sink: &mut dyn MirSink,
    ) -> Result<(), IrError>;
}

pub trait MirToLirLowerer {
    fn lower_mir_to_lir(
        &self,
        program: ValidatedMir<'_>,
        sink: &mut dyn LirSink,
    ) -> Result<(), IrError>;
}

#[cfg(test)]
mod tests {
    use super::{HirToMirLowerer, MirSink};
    use crate::{
        HirNode, HirNodeId, HirNodeKind, HirProgram, IrError, IrLayer, MirOp, ValidatedHir,
    };

    struct RejectingMirSink;

    impl MirSink for RejectingMirSink {
        fn push_mir(&mut self, _op: MirOp) -> Result<(), IrError> {
            Err(IrError::SinkRejected {
                layer: IrLayer::Mir,
            })
        }
    }

    struct EmptyLowerer;

    impl HirToMirLowerer for EmptyLowerer {
        fn lower_hir_to_mir(
            &self,
            _program: ValidatedHir<'_>,
            sink: &mut dyn MirSink,
        ) -> Result<(), IrError> {
            sink.push_mir(MirOp::Exit { code: 0 })
        }
    }

    #[test]
    fn lowering_interfaces_propagate_sink_errors() -> Result<(), IrError> {
        let nodes = [HirNode {
            id: HirNodeId::new(0)?,
            kind: HirNodeKind::Program,
        }];
        let program = ValidatedHir::new(HirProgram { nodes: &nodes })?;
        let mut sink = RejectingMirSink;
        assert_eq!(
            EmptyLowerer.lower_hir_to_mir(program, &mut sink),
            Err(IrError::SinkRejected {
                layer: IrLayer::Mir,
            })
        );
        Ok(())
    }
}
