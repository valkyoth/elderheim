# Elderheim 0.7.0 Release Notes

Status: development

## Scope

`0.7.0` completes the HIR/MIR/LIR core contract stop. It does not add BASIC
lexing, parsing, semantic validation, lowering, executable writing, or runtime
behavior.

## Added

- HIR node and symbol ID newtypes.
- MIR value and label ID newtypes.
- LIR label and symbol ID newtypes.
- Reserved-ID rejection for every IR ID family.
- Borrowed `HirProgram`, `MirProgram`, and `LirProgram` contracts.
- `validate_hir` for non-empty HIR and contiguous HIR node IDs.
- `validate_mir` for MIR labels, values, branches, and terminators.
- `validate_lir` for LIR labels, symbols, references, jumps, and terminators.
- `HirToMirLowerer` and `MirToLirLowerer` traits.
- `MirSink` and `LirSink` traits for allocation-free lowering tests.
- IR contract documentation under `docs/ir-contracts.md`.

## Changed

- `elderheim-ir` was split into focused modules for IDs, HIR, MIR, LIR,
  lowering, and error contracts.

## Scope Exclusions

- BASIC-specific HIR shape remains planned for the BASIC 1 line.
- Real BASIC lowering into MIR remains planned for later stops.
- LIR does not yet encode target instructions or executable relocations.
