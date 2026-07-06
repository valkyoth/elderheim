# IR Contracts

Status: active 0.7.0 contract

`elderheim-ir` owns version-neutral HIR, MIR, and LIR contracts. The crate is
`no_std`, uses borrowed slices for program views, and does not allocate in
production APIs.

## ID Model

The IR layers use explicit ID newtypes:

- HIR: `HirNodeId`, `HirSymbolId`
- MIR: `MirValueId`, `MirLabelId`
- LIR: `LirLabelId`, `LirSymbolId`

All ID constructors reject `u32::MAX` as a reserved sentinel value. IDs expose
their raw value only through `raw()` so validators can report stable failure
details.

## Validated Program Types

Raw `HirProgram`, `MirProgram`, and `LirProgram` values are construction
views. Code that crosses a lowering boundary must use the validated wrappers:

- `ValidatedHir`
- `ValidatedMir`
- `ValidatedLir`

Each wrapper is produced only by a successful validator call. This keeps future
lowering implementations from accidentally accepting unvalidated HIR or MIR.

## HIR Contract

`HirProgram` is a borrowed slice of `HirNode` values. The 0.7.0 validator
checks that a program is non-empty and that node IDs are contiguous from zero.
It intentionally does not define Dartmouth BASIC statement shape yet; that is
reserved for the BASIC 1 HIR stops.

## MIR Contract

MIR is target-neutral. The 0.7.0 contract includes labels, integer constants,
static writes, jumps, conditional branches, and exits. `validate_mir` checks:

- non-empty program;
- maximum program length of `MAX_MIR_OPS`;
- duplicate label rejection;
- duplicate value definition rejection;
- defined branch condition values;
- defined jump/branch labels;
- terminating `Exit`.

`MAX_MIR_OPS` is `65,536`. MIR validation currently uses no allocation and
keeps duplicate/reference checks simple, so the size cap is part of the
security contract.

## LIR Contract

LIR is target-near but not executable bytes. The 0.7.0 contract includes
labels, symbols, symbol references, jumps, and syscall exit markers.
`validate_lir` checks:

- non-empty program;
- maximum program length of `MAX_LIR_OPS`;
- duplicate label rejection;
- duplicate symbol rejection;
- defined jump labels;
- defined referenced symbols;
- terminating `SysExit`.

`MAX_LIR_OPS` is `65,536`. LIR validation currently uses no allocation and is
bounded by this explicit cap.

## Lowering Interfaces

`HirToMirLowerer` and `MirToLirLowerer` define version-neutral lowering
interfaces. They accept `ValidatedHir` and `ValidatedMir`, respectively, and
write to caller-provided `MirSink` and `LirSink` implementations so future
lowerers can be tested without allocating and without committing to a final
backing storage type.

## Verification

The `0.7.0` stop requires:

- ID constructor tests;
- HIR construction and validator failure tests;
- MIR construction and validator failure tests;
- LIR construction and validator failure tests;
- oversized MIR and LIR rejection tests;
- validated-wrapper construction tests;
- IR error code uniqueness tests;
- lowering sink error propagation tests.
