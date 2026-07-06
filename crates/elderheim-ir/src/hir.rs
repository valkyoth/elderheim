use crate::{HirNodeId, IrError, IrLayer};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HirNodeKind {
    Program,
    Statement,
    Expression,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirNode {
    pub id: HirNodeId,
    pub kind: HirNodeKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HirProgram<'a> {
    pub nodes: &'a [HirNode],
}

pub fn validate_hir(program: HirProgram<'_>) -> Result<(), IrError> {
    if program.nodes.is_empty() {
        return Err(IrError::EmptyProgram {
            layer: IrLayer::Hir,
        });
    }

    let mut expected = 0_u32;
    for node in program.nodes {
        if node.id.raw() != expected {
            return Err(IrError::NonContiguousId {
                layer: IrLayer::Hir,
                expected,
                actual: node.id.raw(),
            });
        }
        expected = expected.checked_add(1).ok_or(IrError::ReservedId {
            layer: IrLayer::Hir,
        })?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{HirNode, HirNodeKind, HirProgram, validate_hir};
    use crate::{HirNodeId, IrError, IrLayer};

    #[test]
    fn hir_accepts_contiguous_nodes() -> Result<(), IrError> {
        let nodes = [
            HirNode {
                id: HirNodeId::new(0)?,
                kind: HirNodeKind::Program,
            },
            HirNode {
                id: HirNodeId::new(1)?,
                kind: HirNodeKind::Statement,
            },
        ];
        assert_eq!(validate_hir(HirProgram { nodes: &nodes }), Ok(()));
        Ok(())
    }

    #[test]
    fn hir_rejects_empty_program() {
        assert_eq!(
            validate_hir(HirProgram { nodes: &[] }),
            Err(IrError::EmptyProgram {
                layer: IrLayer::Hir,
            })
        );
    }

    #[test]
    fn hir_rejects_non_contiguous_nodes() -> Result<(), IrError> {
        let nodes = [HirNode {
            id: HirNodeId::new(2)?,
            kind: HirNodeKind::Program,
        }];
        assert_eq!(
            validate_hir(HirProgram { nodes: &nodes }),
            Err(IrError::NonContiguousId {
                layer: IrLayer::Hir,
                expected: 0,
                actual: 2,
            })
        );
        Ok(())
    }
}
