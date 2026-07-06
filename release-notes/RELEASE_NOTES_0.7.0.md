# Elderheim 0.7.0 Release Notes

Status: released

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
- `ValidatedHir`, `ValidatedMir`, and `ValidatedLir` wrappers.
- `validate_hir` for non-empty HIR and contiguous HIR node IDs.
- `validate_mir` for MIR labels, values, branches, and terminators.
- `validate_lir` for LIR labels, symbols, references, jumps, and terminators.
- `HirToMirLowerer` and `MirToLirLowerer` traits.
- `MirSink` and `LirSink` traits for allocation-free lowering tests.
- IR contract documentation under `docs/ir-contracts.md`.

## Changed

- `elderheim-ir` was split into focused modules for IDs, HIR, MIR, LIR,
  lowering, and error contracts.
- MIR and LIR validation now reject programs above 65,536 ops to bound the
  current allocation-free duplicate/reference checks.
- Lowering traits now require validated IR wrapper types instead of raw program
  views.

## Security

- Added regression coverage proving IR diagnostic code strings remain unique.
- Added oversized MIR and LIR rejection tests.

## Scope Exclusions

- BASIC-specific HIR shape remains planned for the BASIC 1 line.
- Real BASIC lowering into MIR remains planned for later stops.
- LIR does not yet encode target instructions or executable relocations.
