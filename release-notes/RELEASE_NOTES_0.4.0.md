# Elderheim 0.4.0 Release Notes

Status: development

## Scope

`0.4.0` completes the compiler pipeline skeleton stop. It does not add BASIC
lexing, parsing, semantic validation, lowering, executable writing, or runtime
behavior.

## Added

- no_std pipeline stage identifiers in `elderheim-core`.
- Ordered source-to-diagnostic, HIR-to-MIR, MIR-to-LIR, and LIR-to-target
  stage boundaries.
- Allocation-free pipeline runner over caller-supplied stage slices.
- Fail-fast stage ordering validation.
- Fail-fast diagnostic error propagation.
- Report sink contract for stage start, stage finish, and diagnostics.
- `NullReportSink` for report-free callers.
- IR boundary markers in `elderheim-ir`.
- Tests for empty pipelines, stage ordering, error propagation, and IR
  boundary mapping.
- Pipeline contract documentation under `docs/pipeline-contract.md`.
- Locked milestone tag guidance changed from `vX.Y.Z-release` to
  `elderheim-vX.Y.Z`.

## Scope Exclusions

- Pipeline stages are contracts only; they do not implement language parsing
  or IR lowering yet.
- Source normalization remains planned for `0.5.0`.
- Diagnostic/report rendering expansion remains planned for `0.6.0`.
