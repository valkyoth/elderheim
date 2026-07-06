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
            Self::DuplicateId { .. } => "E-IR-DUPLICATE-ID",
            Self::NonContiguousId { .. } => "E-IR-NON-CONTIGUOUS-ID",
            Self::UndefinedId { .. } => "E-IR-UNDEFINED-ID",
            Self::MissingTerminator { .. } => "E-IR-MISSING-TERMINATOR",
            Self::SinkRejected { .. } => "E-IR-SINK-REJECTED",
        }
    }
}
