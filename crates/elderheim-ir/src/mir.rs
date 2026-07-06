use crate::{DataId, IrError, IrLayer, MirLabelId, MirValueId};

pub const MAX_MIR_OPS: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MirOp {
    Label(MirLabelId),
    ConstI64 {
        dest: MirValueId,
        value: i64,
    },
    WriteStatic(DataId),
    Jump {
        target: MirLabelId,
    },
    BranchIf {
        condition: MirValueId,
        target: MirLabelId,
    },
    Exit {
        code: u8,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MirProgram<'a> {
    pub ops: &'a [MirOp],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidatedMir<'a> {
    program: MirProgram<'a>,
}

impl<'a> ValidatedMir<'a> {
    pub fn new(program: MirProgram<'a>) -> Result<Self, IrError> {
        validate_mir(program)?;
        Ok(Self { program })
    }

    #[must_use]
    pub const fn program(self) -> MirProgram<'a> {
        self.program
    }
}

pub fn validate_mir(program: MirProgram<'_>) -> Result<(), IrError> {
    if program.ops.is_empty() {
        return Err(IrError::EmptyProgram {
            layer: IrLayer::Mir,
        });
    }
    if program.ops.len() > MAX_MIR_OPS {
        return Err(IrError::ProgramTooLarge {
            layer: IrLayer::Mir,
            len: len_to_u32(program.ops.len()),
            max: len_to_u32(MAX_MIR_OPS),
        });
    }

    for op in program.ops {
        validate_mir_op(*op, program.ops)?;
    }

    if matches!(program.ops.last(), Some(MirOp::Exit { .. })) {
        Ok(())
    } else {
        Err(IrError::MissingTerminator {
            layer: IrLayer::Mir,
        })
    }
}

fn validate_mir_op(op: MirOp, ops: &[MirOp]) -> Result<(), IrError> {
    match op {
        MirOp::Label(label) => reject_duplicate_label(label, ops),
        MirOp::ConstI64 { dest, .. } => reject_duplicate_value(dest, ops),
        MirOp::Jump { target } => require_label(target, ops),
        MirOp::BranchIf { condition, target } => {
            require_value(condition, ops)?;
            require_label(target, ops)
        }
        MirOp::WriteStatic(_) | MirOp::Exit { .. } => Ok(()),
    }
}

fn reject_duplicate_label(label: MirLabelId, ops: &[MirOp]) -> Result<(), IrError> {
    let mut seen = false;
    for op in ops {
        if *op == MirOp::Label(label) {
            if seen {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Mir,
                    id: label.raw(),
                });
            }
            seen = true;
        }
    }
    Ok(())
}

fn reject_duplicate_value(value: MirValueId, ops: &[MirOp]) -> Result<(), IrError> {
    let mut seen = false;
    for op in ops {
        if matches!(*op, MirOp::ConstI64 { dest, .. } if dest == value) {
            if seen {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Mir,
                    id: value.raw(),
                });
            }
            seen = true;
        }
    }
    Ok(())
}

fn require_label(label: MirLabelId, ops: &[MirOp]) -> Result<(), IrError> {
    if ops.contains(&MirOp::Label(label)) {
        Ok(())
    } else {
        Err(IrError::UndefinedId {
            layer: IrLayer::Mir,
            id: label.raw(),
        })
    }
}

fn require_value(value: MirValueId, ops: &[MirOp]) -> Result<(), IrError> {
    if ops
        .iter()
        .any(|op| matches!(*op, MirOp::ConstI64 { dest, .. } if dest == value))
    {
        Ok(())
    } else {
        Err(IrError::UndefinedId {
            layer: IrLayer::Mir,
            id: value.raw(),
        })
    }
}

fn len_to_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::{MAX_MIR_OPS, MirOp, MirProgram, ValidatedMir, validate_mir};
    use crate::{DataId, IrError, IrLayer, MirLabelId, MirValueId};

    #[test]
    fn mir_accepts_target_neutral_program() -> Result<(), IrError> {
        let ops = [
            MirOp::Label(MirLabelId::new(0)?),
            MirOp::ConstI64 {
                dest: MirValueId::new(0)?,
                value: 42,
            },
            MirOp::WriteStatic(DataId::new(0)?),
            MirOp::Exit { code: 0 },
        ];
        assert_eq!(validate_mir(MirProgram { ops: &ops }), Ok(()));
        Ok(())
    }

    #[test]
    fn mir_rejects_missing_terminator() -> Result<(), IrError> {
        let ops = [MirOp::Label(MirLabelId::new(0)?)];
        assert_eq!(
            validate_mir(MirProgram { ops: &ops }),
            Err(IrError::MissingTerminator {
                layer: IrLayer::Mir,
            })
        );
        Ok(())
    }

    #[test]
    fn mir_rejects_undefined_branch_value() -> Result<(), IrError> {
        let ops = [
            MirOp::Label(MirLabelId::new(0)?),
            MirOp::BranchIf {
                condition: MirValueId::new(7)?,
                target: MirLabelId::new(0)?,
            },
            MirOp::Exit { code: 0 },
        ];
        assert_eq!(
            validate_mir(MirProgram { ops: &ops }),
            Err(IrError::UndefinedId {
                layer: IrLayer::Mir,
                id: 7,
            })
        );
        Ok(())
    }

    #[test]
    fn mir_rejects_oversized_programs() {
        static OPS: [MirOp; MAX_MIR_OPS + 1] = [MirOp::Exit { code: 0 }; MAX_MIR_OPS + 1];
        assert_eq!(
            validate_mir(MirProgram { ops: &OPS }),
            Err(IrError::ProgramTooLarge {
                layer: IrLayer::Mir,
                len: 65_537,
                max: 65_536,
            })
        );
    }

    #[test]
    fn validated_mir_requires_successful_validation() -> Result<(), IrError> {
        let ops = [MirOp::Exit { code: 0 }];
        let program = MirProgram { ops: &ops };
        assert_eq!(ValidatedMir::new(program)?.program(), program);
        assert_eq!(
            ValidatedMir::new(MirProgram { ops: &[] }),
            Err(IrError::EmptyProgram {
                layer: IrLayer::Mir,
            })
        );
        Ok(())
    }
}
