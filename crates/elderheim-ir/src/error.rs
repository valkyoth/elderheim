#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrLayer {
    Hir,
    Mir,
    Lir,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrError {
    ReservedId {
        layer: IrLayer,
    },
    EmptyProgram {
        layer: IrLayer,
    },
    ProgramTooLarge {
        layer: IrLayer,
        len: u32,
        max: u32,
    },
    DuplicateId {
        layer: IrLayer,
        id: u32,
    },
    NonContiguousId {
        layer: IrLayer,
        expected: u32,
        actual: u32,
    },
    UndefinedId {
        layer: IrLayer,
        id: u32,
    },
    MissingTerminator {
        layer: IrLayer,
    },
    SinkRejected {
        layer: IrLayer,
    },
}

impl IrError {
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::ReservedId { .. } => "E-IR-RESERVED-ID",
            Self::EmptyProgram { .. } => "E-IR-EMPTY-PROGRAM",
            Self::ProgramTooLarge { .. } => "E-IR-PROGRAM-TOO-LARGE",
            Self::DuplicateId { .. } => "E-IR-DUPLICATE-ID",
            Self::NonContiguousId { .. } => "E-IR-NON-CONTIGUOUS-ID",
            Self::UndefinedId { .. } => "E-IR-UNDEFINED-ID",
            Self::MissingTerminator { .. } => "E-IR-MISSING-TERMINATOR",
            Self::SinkRejected { .. } => "E-IR-SINK-REJECTED",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IrError, IrLayer};

    #[test]
    fn ir_error_codes_are_unique() {
        let codes = [
            IrError::ReservedId {
                layer: IrLayer::Hir,
            }
            .code(),
            IrError::EmptyProgram {
                layer: IrLayer::Hir,
            }
            .code(),
            IrError::ProgramTooLarge {
                layer: IrLayer::Hir,
                len: 0,
                max: 0,
            }
            .code(),
            IrError::DuplicateId {
                layer: IrLayer::Hir,
                id: 0,
            }
            .code(),
            IrError::NonContiguousId {
                layer: IrLayer::Hir,
                expected: 0,
                actual: 0,
            }
            .code(),
            IrError::UndefinedId {
                layer: IrLayer::Hir,
                id: 0,
            }
            .code(),
            IrError::MissingTerminator {
                layer: IrLayer::Hir,
            }
            .code(),
            IrError::SinkRejected {
                layer: IrLayer::Hir,
            }
            .code(),
        ];

        for (index, code) in codes.iter().enumerate() {
            for other in codes.iter().skip(index + 1) {
                assert_ne!(code, other, "duplicate IrError code: {code}");
            }
        }
    }
}
