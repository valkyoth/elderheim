# Elderheim 0.3.0 Release Notes

Status: development

## Scope

`0.3.0` completes the source and diagnostics core stop. It does not add BASIC
lexing, parsing, semantic validation, lowering, executable writing, or runtime
behavior.

## Added

- no_std source byte model in `elderheim-core`.
- Checked half-open source spans with private fields.
- One-based byte-offset to line/column lookup.
- Cursor-based line/column lookup for batched diagnostics.
- Source byte limit enforcement.
- Source line limit enforcement.
- Stable core diagnostic codes.
- Diagnostic severity labels.
- Compact diagnostic rendering contract.
- Golden tests for diagnostic rendering.
- Limit tests for source byte and source line bounds.
- Source and diagnostics contract documentation under
  `docs/source-diagnostics.md`.
- Release-readiness remote branch verification preserves `git ls-remote`
  failures without a pipeline and without a temporary file.

## Security Review Fixes

- Added `LineCursor` and `Diagnostic::render_with_cursor` so left-to-right
  diagnostic rendering can avoid repeated full-source rescans.
- Moved the maximum supported source length check before line counting.
- Removed the predictable temporary file from
  `scripts/validate-release-readiness.sh`.
- Made invalid reversed spans unrepresentable through the public span API.

## Scope Exclusions

- Source normalization and profile-specific byte validation are planned for
  later compiler-substrate stops.
- Dartmouth BASIC lexing and parsing begin only after the compiler substrate
  stops are complete.
- Executable output remains out of scope for this tag.
