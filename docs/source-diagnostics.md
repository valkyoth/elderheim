# Source And Diagnostics Core

Status: active 0.6.0 contract

`elderheim-core` owns source-byte handling, spans, compile limits, and stable
diagnostics for all future compiler phases. This crate remains `no_std` and
does not allocate in production APIs.

## Source Bytes

`Source::from_normalized` accepts borrowed normalized source bytes, configured
`CompileLimits`, and a `NormalizationPolicy`. It checks:

- whether source length fits the supported `u32` span model;
- total source byte length;
- total line count;
- printable-ASCII normalized source byte policy;
- selected blank-line policy.

The public `Source` constructor is for already-normalized bytes only. Raw file
bytes must first pass through `normalize_source`; later language frontends then
apply profile-specific lexical rules.

## Locations

Line and column values are one-based. Offset `0` is line `1`, column `1`.
The EOF offset is valid and maps to the column after the final byte.

Line counting treats normalized `\n` as the line separator. A trailing newline
creates a final empty line for limit accounting, but it is not treated as a
blank source line by the strict BASIC normalization policy.

`Source::line_column` is the one-off lookup path. Batched diagnostics should
use `LineCursor` through `Source::line_column_from` or
`Diagnostic::render_with_cursor`. Forward lookups resume from the previous
offset and keep diagnostic rendering linear over a left-to-right pass. An
out-of-order lookup resets the cursor and remains correct.

## Spans

`Span` uses half-open byte offsets:

```text
start <= offset < end
```

Span fields are private. Callers must use `Span::checked`,
`Span::from_start_len`, or `Span::point`, so reversed spans cannot be built
through the public API. Empty spans are valid and are used for point
diagnostics.

## Diagnostics

Diagnostics carry:

- a stable diagnostic code;
- severity;
- source span;
- a stable message selected by code.

The compact rendering contract is:

```text
<severity> <code> <line>:<column> <message>
```

The snippet rendering contract is documented in
[`diagnostics-reporting.md`](diagnostics-reporting.md).

When no source is available, the renderer uses `0:0` as the location. When a
source is supplied but the diagnostic span cannot be resolved against that
source, rendering emits `E-CORE-INTERNAL-LOCATION` instead of silently
fabricating a source location.
Batch rendering with source context should use `Diagnostic::render_with_cursor`
so repeated diagnostics do not rescan the source from byte zero.

## Current Stable Codes

| Code | Meaning |
| --- | --- |
| `E-CORE-UNSUPPORTED-FEATURE` | selected compiler path does not support the feature |
| `E-CORE-INVALID-DIALECT` | selected language dialect is not recognized |
| `E-CORE-PROGRAM-SIZE` | program exceeds configured compile limits |
| `E-CORE-EXECUTABLE-LAYOUT` | executable layout is invalid |
| `E-CORE-SOURCE-SIZE` | source exceeds configured byte limit |
| `E-CORE-SOURCE-LINES` | source exceeds configured line limit |
| `E-CORE-SOURCE-SPAN` | source span is invalid |
| `E-CORE-SOURCE-OFFSET` | source offset is outside the source byte range |
| `E-CORE-SOURCE-BYTE` | source byte is outside the current source policy |
| `E-CORE-SOURCE-BLANK-LINE` | source blank line is rejected by policy |
| `E-CORE-INTERNAL-LOCATION` | diagnostic location could not be resolved |

## Verification

The `0.6.0` stop requires:

- span lookup tests;
- cursor lookup tests;
- diagnostic golden tests;
- diagnostic registry tests;
- snippet diagnostic golden tests;
- malformed snippet tests;
- cursor diagnostic rendering golden tests;
- source byte limit tests;
- source line limit tests.
