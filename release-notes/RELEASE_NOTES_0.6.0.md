# Elderheim 0.6.0 Release Notes

Status: development

## Scope

`0.6.0` completes the diagnostics and reporting core stop. It does not add
BASIC lexing, parsing, semantic validation, lowering, executable writing, or
runtime behavior.

## Added

- Stable diagnostic code registry through `DiagnosticCode::ALL`.
- `DiagnosticDescriptor` values for code, identifier, message, and default
  severity.
- Source snippet diagnostic rendering through `RenderStyle::Snippet`.
- Explicit malformed-snippet handling with `E-CORE-INTERNAL-LOCATION`.
- Optional source-ID binding for diagnostics.
- Cursor source binding and cursor-backed snippet line extraction.
- `ReportSection` keys for summary, pipeline, and diagnostics output.
- `ReportEvent` rendering for report sections, pipeline starts, pipeline
  finishes, and diagnostics.
- `StageOutcome::label` for stable report rendering.
- Golden tests for diagnostic registry, snippets, malformed snippets, report
  sections, report stage events, and report diagnostics.
- Diagnostics/reporting documentation under `docs/diagnostics-reporting.md`.

## Security Review Fixes

- Cursor-backed snippet rendering now uses the cursor's resolved line-start
  offset and copies only the target line.
- `LineCursor` is bound to the source slice it was advanced against and resets
  when used with another source.
- Source-bound diagnostics now fail closed when rendered with the wrong source.
- Diagnostic registry tests now verify the registry length against an
  exhaustive variant count and reject duplicate entries.

## Changed

- Diagnostic tests moved into `crates/elderheim-core/src/diagnostic/tests.rs`
  to keep the production diagnostic module below the file-size ceiling.

## Scope Exclusions

- The report model does not define a final CLI report format yet.
- Snippet rendering is source-line oriented only; multi-span diagnostics remain
  planned for later language frontend work.
- Parser diagnostics remain planned for the Dartmouth BASIC implementation
  line.
