# Elderheim 0.5.0 Release Notes

Status: development

## Scope

`0.5.0` completes the source normalization core stop. It does not add BASIC
lexing, parsing, semantic validation, lowering, executable writing, or runtime
behavior.

## Added

- no_std source normalization in `elderheim-core`.
- Caller-provided normalized source sink contract.
- LF, CRLF, and CR line-ending normalization to LF.
- Strict printable-ASCII source byte policy.
- Invalid control/non-ASCII byte diagnostics.
- Strict BASIC blank-line rejection policy.
- Optional blank-line preservation policy for tests and future profiles.
- Stable `SourceId` over normalized bytes.
- Public `Source::from_normalized` construction that rejects unnormalized or
  policy-invalid source bytes.
- Shared normalized-byte policy scanner used by both `normalize_source` and
  `Source::from_normalized`.
- Visible internal diagnostic-location failure reporting.
- Tests for line-ending normalization, invalid byte rejection, blank-line
  policy, source-size rejection, source ID stability, public source
  construction rejection, and diagnostic location-failure rendering.
- Source normalization documentation under `docs/source-normalization.md`.
- Refreshed the project overview image under `.github/images/elderheim.webp`.

## Scope Exclusions

- Source normalization does not parse Dartmouth BASIC line numbers.
- Profile-specific lexical validation remains a later frontend concern.
- Diagnostic/report rendering expansion remains planned for `0.6.0`.
