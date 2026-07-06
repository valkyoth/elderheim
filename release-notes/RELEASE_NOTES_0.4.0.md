# Elderheim 0.4.0 Release Notes

Status: release-candidate ready

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
- Fail-fast missing-stage validation.
- Fail-fast diagnostic error propagation.
- Report sink contract for stage start, stage finish, and diagnostics.
- `NullReportSink` for report-free callers.
- IR boundary markers in `elderheim-ir`.
- Tests for empty pipelines, stage ordering, error propagation, and IR
  boundary mapping.
- Pipeline contract documentation under `docs/pipeline-contract.md`.
- Locked milestone tag guidance changed from `vX.Y.Z-release` to
  `elderheim-vX.Y.Z`.
- Release tag validators now require strict numeric `X.Y.Z` version segments.
- `scripts/checks.sh` now enforces `cargo deny check` and `cargo audit`.

## Security Review Fixes

- Pipeline ordering validation now rejects skipped intermediate stages.
- Pipeline code was split into smaller stage, report, and runner modules.
- Release tag validation rejects path separators and non-numeric version
  segments before building release-note or pentest-report paths.
- Supply-chain checks are part of the standard local gate.
- GitHub CI installs the required supply-chain tools before running the
  standard local gate.

## Scope Exclusions

- Pipeline stages are contracts only; they do not implement language parsing
  or IR lowering yet.
- Source normalization remains planned for `0.5.0`.
- Diagnostic/report rendering expansion remains planned for `0.6.0`.
