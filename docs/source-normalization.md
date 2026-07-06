# Source Normalization

Status: active 0.5.0 contract

`elderheim-core` owns source normalization before language parsing starts. The
normalizer is `no_std` and writes normalized bytes to a caller-provided sink
instead of allocating.

## Line Endings

Input line endings are normalized to `\n`.

| Input | Output |
| --- | --- |
| LF | LF |
| CRLF | LF |
| CR | LF |

The normalized stream is the input to later lexing and diagnostics.

## Byte Policy

The current policy accepts printable ASCII bytes `0x20..=0x7e` plus line
endings. Other bytes fail with `E-CORE-SOURCE-BYTE`.

This is a compiler-foundation policy, not a Dartmouth BASIC grammar. Later
language profiles may add profile-specific source restrictions after
normalization.

## Blank Lines

`NormalizationPolicy::BASIC_STRICT` rejects blank normalized lines with
`E-CORE-SOURCE-BLANK-LINE`. A line containing only spaces is blank.

`NormalizationPolicy::PRESERVE_BLANK_LINES` keeps blank lines for tests and
future non-BASIC source policies.

## Limits

The normalizer checks the configured source byte limit before writing to the
sink. It also enforces the configured line limit after line-ending
normalization.

## Source IDs

`SourceId` is computed over normalized bytes. Inputs that differ only by LF,
CRLF, or CR line endings therefore produce the same source ID.

The ID is intended as a stable compiler-internal identity for reports and
fixtures. It is not a cryptographic digest.

## Verification

The `0.5.0` stop requires:

- LF, CRLF, and CR normalization fixtures;
- invalid byte fixtures;
- blank-line policy fixtures;
- large source rejection tests;
- source ID stability tests.
