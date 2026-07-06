# Diagnostics And Reporting

Status: active 0.6.0 contract

`elderheim-core` owns diagnostic identity, diagnostic rendering, and early
compiler report rendering before language parsing starts. The APIs remain
`no_std` and render into caller-provided `core::fmt::Write` sinks.

## Diagnostic Registry

`DiagnosticCode::ALL` is the stable registry for current core diagnostic
codes. Each code exposes a `DiagnosticDescriptor` containing:

- the enum value;
- the stable string identifier;
- the stable message;
- the default severity.

New codes must be added deliberately and covered by a registry golden test.
The registry test checks the hand-maintained registry against an exhaustive
variant count and rejects duplicate registry entries.

## Diagnostic Rendering

`RenderStyle::Compact` renders one stable line:

```text
<severity> <code> <line>:<column> <message>
```

`RenderStyle::Snippet` renders the compact line followed by a source excerpt
when source context is available:

```text
error E-CORE-UNSUPPORTED-FEATURE 2:4 feature is not supported by the selected compiler path
 --> 2:4
  |
2 | 20 GOTO 10
  |    ^
```

When no source is supplied, snippet rendering falls back to the compact `0:0`
location. When source is supplied but the span cannot be resolved, rendering
emits `E-CORE-INTERNAL-LOCATION` instead of fabricating a source location.
Cursor-backed snippet rendering uses the cursor's resolved line-start offset,
so repeated snippet diagnostics do not rescan source bytes from the beginning.

Diagnostics may carry a normalized `SourceId`. When present, the renderer uses
that ID as best-effort misuse detection inside one trusted compilation
session. If the supplied `Source` has a different ID, rendering emits
`E-CORE-INTERNAL-LOCATION`. This is not an adversarial source-equality
guarantee and does not make `SourceId` a security boundary.

## Report Sections

`ReportSection` defines the stable report section keys:

- `summary`
- `pipeline`
- `diagnostics`

`ReportEvent` renders stable lines for section headers, pipeline stage starts,
pipeline stage finishes, and diagnostics. The output is intentionally simple so
future CLI, JSON, and artifact formats can be built on top of a small tested
core contract.

## Verification

The `0.6.0` stop requires:

- diagnostic registry golden tests;
- compact diagnostic golden tests;
- snippet diagnostic golden tests;
- malformed snippet failure tests;
- source-bound diagnostic mismatch tests;
- report section golden tests;
- report event golden tests.
