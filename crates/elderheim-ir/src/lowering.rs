use crate::{HirProgram, IrError, LirOp, MirOp, MirProgram};

pub trait MirSink {
    fn push_mir(&mut self, op: MirOp) -> Result<(), IrError>;
}

pub trait LirSink {
    fn push_lir(&mut self, op: LirOp) -> Result<(), IrError>;
}

pub trait HirToMirLowerer {
    fn lower_hir_to_mir(
        &self,
        program: HirProgram<'_>,
        sink: &mut dyn MirSink,
    ) -> Result<(), IrError>;
}

pub trait MirToLirLowerer {
    fn lower_mir_to_lir(
        &self,
        program: MirProgram<'_>,
        sink: &mut dyn LirSink,
    ) -> Result<(), IrError>;
}

#[cfg(test)]
mod tests {
    use super::{HirToMirLowerer, MirSink};
    use crate::{HirProgram, IrError, IrLayer, MirOp};

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
            _program: HirProgram<'_>,
            sink: &mut dyn MirSink,
        ) -> Result<(), IrError> {
            sink.push_mir(MirOp::Exit { code: 0 })
        }
    }

    #[test]
    fn lowering_interfaces_propagate_sink_errors() {
        let mut sink = RejectingMirSink;
        assert_eq!(
            EmptyLowerer.lower_hir_to_mir(HirProgram { nodes: &[] }, &mut sink),
            Err(IrError::SinkRejected {
                layer: IrLayer::Mir,
            })
        );
    }
}
