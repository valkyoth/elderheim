# Elderheim 0.9.0 Release Notes

Status: released

## Scope

`0.9.0` completes the manual corpus setup stop. It adds the first
Elderheim-authored Dartmouth BASIC 1 language reference, controlled BASIC 1
example sources, and validation that the committed corpus stays present and
testable.

This release does not add a BASIC lexer, parser, semantic validator, MIR
lowering, runtime execution, or executable output.

## Added

- `docs/languages/` for language-specific Elderheim-authored references.
- `docs/languages/dartmouth-basic-1.md` for the BASIC 1 corpus surface.
- `docs/manual-corpus.md` for local manual provenance and commit policy.
- `examples/dartmouth-basic-1/` with BASIC 1 source examples.
- `examples/dartmouth-basic-1/manifest.txt` for committed fixture tracking.
- `validate_basic1_corpus_source` in the Dartmouth BASIC crate.
- Tests that include and validate every committed BASIC 1 example.
- Tests rejecting session commands, unordered line numbers, and missing final
  `END`.
- `scripts/validate-manual-corpus.sh` for docs/examples/manual-path checks.

## Security

- Manual PDFs remain local provenance inputs and are not committed.
- Missing local manual paths warn instead of failing CI.
- BASIC 1 corpus validation rejects historical session commands as source.
- BASIC 1 corpus validation now enforces a keyword boundary for `PRINT`.

## Scope Exclusions

- BASIC 1 parsing starts in the `v0.10.0-elderheim` line-table stop.
- Dartmouth BASIC 2 and 4 language reference documents remain future corpus
  work.
- Dartmouth BASIC 3 remains reserved until official documentation is available.
