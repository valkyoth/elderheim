# Elderheim 0.3.0 Release Notes

Status: development

## Scope

`0.3.0` completes the source and diagnostics core stop. It does not add BASIC
lexing, parsing, semantic validation, lowering, executable writing, or runtime
behavior.

## Added

- no_std source byte model in `elderheim-core`.
- Checked half-open source spans.
- One-based byte-offset to line/column lookup.
- Source byte limit enforcement.
- Source line limit enforcement.
- Stable core diagnostic codes.
- Diagnostic severity labels.
- Compact diagnostic rendering contract.
- Golden tests for diagnostic rendering.
- Limit tests for source byte and source line bounds.
- Source and diagnostics contract documentation under
  `docs/source-diagnostics.md`.
- Release-readiness remote branch verification hardened to preserve
  `git ls-remote` failures without a pipeline or command substitution.

## Scope Exclusions

- Source normalization and profile-specific byte validation are planned for
  later compiler-substrate stops.
- Dartmouth BASIC lexing and parsing begin only after the compiler substrate
  stops are complete.
- Executable output remains out of scope for this tag.
