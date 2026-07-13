use alloc::collections::BTreeSet;

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

    let mut labels = BTreeSet::new();
    let mut values = BTreeSet::new();
    for op in program.ops {
        match *op {
            MirOp::Label(label) if !labels.insert(label) => {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Mir,
                    id: label.raw(),
                });
            }
            MirOp::ConstI64 { dest, .. } if !values.insert(dest) => {
                return Err(IrError::DuplicateId {
                    layer: IrLayer::Mir,
                    id: dest.raw(),
                });
            }
            _ => {}
        }
    }

    for op in program.ops {
        match *op {
            MirOp::Jump { target } if !labels.contains(&target) => {
                return Err(undefined(target.raw()));
            }
            MirOp::BranchIf { condition, target } => {
                if !values.contains(&condition) {
                    return Err(undefined(condition.raw()));
                }
                if !labels.contains(&target) {
                    return Err(undefined(target.raw()));
                }
            }
            _ => {}
        }
    }

    if matches!(program.ops.last(), Some(MirOp::Exit { .. })) {
        Ok(())
    } else {
        Err(IrError::MissingTerminator {
            layer: IrLayer::Mir,
        })
    }
}

const fn undefined(id: u32) -> IrError {
    IrError::UndefinedId {
        layer: IrLayer::Mir,
        id,
    }
}

fn len_to_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::{MAX_MIR_OPS, MirOp, MirProgram, ValidatedMir, validate_mir};
    use crate::{DataId, IrError, IrLayer, MirLabelId, MirValueId};
    use alloc::vec::Vec;

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
    fn mir_rejects_undefined_jump_and_branch_labels() -> Result<(), IrError> {
        let undefined = MirLabelId::new(8)?;
        let jump = [MirOp::Jump { target: undefined }, MirOp::Exit { code: 0 }];
        assert_eq!(
            validate_mir(MirProgram { ops: &jump }),
            Err(IrError::UndefinedId {
                layer: IrLayer::Mir,
                id: 8,
            })
        );

        let branch = [
            MirOp::ConstI64 {
                dest: MirValueId::new(0)?,
                value: 1,
            },
            MirOp::BranchIf {
                condition: MirValueId::new(0)?,
                target: undefined,
            },
            MirOp::Exit { code: 0 },
        ];
        assert_eq!(
            validate_mir(MirProgram { ops: &branch }),
            Err(IrError::UndefinedId {
                layer: IrLayer::Mir,
                id: 8,
            })
        );
        Ok(())
    }

    #[test]
    fn mir_rejects_duplicate_labels_and_values() -> Result<(), IrError> {
        let label = MirLabelId::new(3)?;
        let duplicate_labels = [
            MirOp::Label(label),
            MirOp::Label(label),
            MirOp::Exit { code: 0 },
        ];
        assert_eq!(
            validate_mir(MirProgram {
                ops: &duplicate_labels,
            }),
            Err(IrError::DuplicateId {
                layer: IrLayer::Mir,
                id: 3,
            })
        );

        let value = MirValueId::new(4)?;
        let duplicate_values = [
            MirOp::ConstI64 {
                dest: value,
                value: 1,
            },
            MirOp::ConstI64 {
                dest: value,
                value: 2,
            },
            MirOp::Exit { code: 0 },
        ];
        assert_eq!(
            validate_mir(MirProgram {
                ops: &duplicate_values,
            }),
            Err(IrError::DuplicateId {
                layer: IrLayer::Mir,
                id: 4,
            })
        );
        Ok(())
    }

    #[test]
    fn mir_validates_maximum_unique_label_program() -> Result<(), IrError> {
        let mut ops = Vec::with_capacity(MAX_MIR_OPS);
        for index in 0..MAX_MIR_OPS.saturating_sub(1) {
            let raw = u32::try_from(index).unwrap_or(u32::MAX);
            ops.push(MirOp::Label(MirLabelId::new(raw)?));
        }
        ops.push(MirOp::Exit { code: 0 });

        assert_eq!(ops.len(), MAX_MIR_OPS);
        assert_eq!(validate_mir(MirProgram { ops: &ops }), Ok(()));
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
