use crate::{IrError, IrLayer, LirLabelId, LirSymbolId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LirOp {
    Label(LirLabelId),
    DefineSymbol(LirSymbolId),
    ReferenceSymbol(LirSymbolId),
    Jump { target: LirLabelId },
    SysExit { code: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LirProgram<'a> {
    pub ops: &'a [LirOp],
}

pub fn validate_lir(program: LirProgram<'_>) -> Result<(), IrError> {
    if program.ops.is_empty() {
        return Err(IrError::EmptyProgram {
            layer: IrLayer::Lir,
        });
    }

    for op in program.ops {
        validate_lir_op(*op, program.ops)?;
    }

    if matches!(program.ops.last(), Some(LirOp::SysExit { .. })) {
        Ok(())
    } else {
        Err(IrError::MissingTerminator {
            layer: IrLayer::Lir,
        })
    }
}

fn validate_lir_op(op: LirOp, ops: &[LirOp]) -> Result<(), IrError> {
    match op {
        LirOp::Label(label) => reject_duplicate_label(label, ops),
        LirOp::DefineSymbol(symbol) => reject_duplicate_symbol(symbol, ops),
        LirOp::ReferenceSymbol(symbol) => require_symbol(symbol, ops),
        LirOp::Jump { target } => require_label(target, ops),
        LirOp::SysExit { .. } => Ok(()),
    }
}

fn reject_duplicate_label(label: LirLabelId, ops: &[LirOp]) -> Result<(), IrError> {
    let mut seen = false;
    for op in ops {
        if *op == LirOp::Label(label) {
            if seen {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Lir,
                    id: label.raw(),
                });
            }
            seen = true;
        }
    }
    Ok(())
}

fn reject_duplicate_symbol(symbol: LirSymbolId, ops: &[LirOp]) -> Result<(), IrError> {
    let mut seen = false;
    for op in ops {
        if *op == LirOp::DefineSymbol(symbol) {
            if seen {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Lir,
                    id: symbol.raw(),
                });
            }
            seen = true;
        }
    }
    Ok(())
}

fn require_label(label: LirLabelId, ops: &[LirOp]) -> Result<(), IrError> {
    if ops.contains(&LirOp::Label(label)) {
        Ok(())
    } else {
        Err(IrError::UndefinedId {
            layer: IrLayer::Lir,
            id: label.raw(),
        })
    }
}

fn require_symbol(symbol: LirSymbolId, ops: &[LirOp]) -> Result<(), IrError> {
    if ops.contains(&LirOp::DefineSymbol(symbol)) {
        Ok(())
    } else {
        Err(IrError::UndefinedId {
            layer: IrLayer::Lir,
            id: symbol.raw(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{LirOp, LirProgram, validate_lir};
    use crate::{IrError, IrLayer, LirLabelId, LirSymbolId};

    #[test]
    fn lir_accepts_target_near_program() -> Result<(), IrError> {
        let ops = [
            LirOp::Label(LirLabelId::new(0)?),
            LirOp::DefineSymbol(LirSymbolId::new(0)?),
            LirOp::ReferenceSymbol(LirSymbolId::new(0)?),
            LirOp::SysExit { code: 0 },
        ];
        assert_eq!(validate_lir(LirProgram { ops: &ops }), Ok(()));
        Ok(())
    }

    #[test]
    fn lir_rejects_undefined_jump_label() -> Result<(), IrError> {
        let ops = [
            LirOp::Jump {
                target: LirLabelId::new(9)?,
            },
            LirOp::SysExit { code: 0 },
        ];
        assert_eq!(
            validate_lir(LirProgram { ops: &ops }),
            Err(IrError::UndefinedId {
                layer: IrLayer::Lir,
                id: 9,
            })
        );
        Ok(())
    }

    #[test]
    fn lir_rejects_duplicate_symbol() -> Result<(), IrError> {
        let ops = [
            LirOp::DefineSymbol(LirSymbolId::new(1)?),
            LirOp::DefineSymbol(LirSymbolId::new(1)?),
            LirOp::SysExit { code: 0 },
        ];
        assert_eq!(
            validate_lir(LirProgram { ops: &ops }),
            Err(IrError::DuplicateId {
                layer: IrLayer::Lir,
                id: 1,
            })
        );
        Ok(())
    }
}
