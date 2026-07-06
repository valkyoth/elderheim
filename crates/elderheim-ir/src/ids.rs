use crate::{IrError, IrLayer};

macro_rules! id_type {
    ($name:ident, $layer:expr) => {
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            pub const RESERVED: u32 = u32::MAX;

            pub const fn new(value: u32) -> Result<Self, IrError> {
                if value == Self::RESERVED {
                    Err(IrError::ReservedId { layer: $layer })
                } else {
                    Ok(Self(value))
                }
            }

            #[must_use]
            pub const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

id_type!(HirNodeId, IrLayer::Hir);
id_type!(HirSymbolId, IrLayer::Hir);
id_type!(MirValueId, IrLayer::Mir);
id_type!(MirLabelId, IrLayer::Mir);
id_type!(LirLabelId, IrLayer::Lir);
id_type!(LirSymbolId, IrLayer::Lir);
id_type!(DataId, IrLayer::Mir);

#[cfg(test)]
mod tests {
    use super::{HirNodeId, LirLabelId, MirValueId};
    use crate::{IrError, IrLayer};

    #[test]
    fn ids_reject_reserved_value() {
        assert_eq!(
            HirNodeId::new(u32::MAX),
            Err(IrError::ReservedId {
                layer: IrLayer::Hir,
            })
        );
        assert_eq!(
            MirValueId::new(u32::MAX),
            Err(IrError::ReservedId {
                layer: IrLayer::Mir,
            })
        );
        assert_eq!(
            LirLabelId::new(u32::MAX),
            Err(IrError::ReservedId {
                layer: IrLayer::Lir,
            })
        );
    }
}
