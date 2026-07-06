use crate::{IrError, IrLayer, LirLabelId, LirSymbolId};

pub const MAX_LIR_OPS: usize = 64 * 1024;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedLir<'a> {
    program: LirProgram<'a>,
}

impl<'a> ValidatedLir<'a> {
    pub fn new(program: LirProgram<'a>) -> Result<Self, IrError> {
        validate_lir(program)?;
        Ok(Self { program })
    }

    #[must_use]
    pub const fn program(self) -> LirProgram<'a> {
        self.program
    }
}

pub fn validate_lir(program: LirProgram<'_>) -> Result<(), IrError> {
    if program.ops.is_empty() {
        return Err(IrError::EmptyProgram {
            layer: IrLayer::Lir,
        });
    }
    if program.ops.len() > MAX_LIR_OPS {
        return Err(IrError::ProgramTooLarge {
            layer: IrLayer::Lir,
            len: len_to_u32(program.ops.len()),
            max: len_to_u32(MAX_LIR_OPS),
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

fn len_to_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::{LirOp, LirProgram, MAX_LIR_OPS, ValidatedLir, validate_lir};
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

    #[test]
    fn lir_rejects_oversized_programs() {
        static OPS: [LirOp; MAX_LIR_OPS + 1] = [LirOp::SysExit { code: 0 }; MAX_LIR_OPS + 1];
        assert_eq!(
            validate_lir(LirProgram { ops: &OPS }),
            Err(IrError::ProgramTooLarge {
                layer: IrLayer::Lir,
                len: 65_537,
                max: 65_536,
            })
        );
    }

    #[test]
    fn validated_lir_requires_successful_validation() -> Result<(), IrError> {
        let ops = [LirOp::SysExit { code: 0 }];
        let program = LirProgram { ops: &ops };
        assert_eq!(ValidatedLir::new(program)?.program(), program);
        assert_eq!(
            ValidatedLir::new(LirProgram { ops: &[] }),
            Err(IrError::EmptyProgram {
                layer: IrLayer::Lir,
            })
        );
        Ok(())
    }
}
