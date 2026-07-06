# Pipeline Contract

Status: active 0.4.0 contract

`elderheim-core` owns the non-language-specific compiler pipeline skeleton.
The contract is intentionally small, `no_std`, and allocation-free for
production callers.

## Stage Order

Pipeline stages have a fixed order:

1. `source-to-diagnostic`
2. `hir-to-mir`
3. `mir-to-lir`
4. `lir-to-target`

`run_pipeline` rejects duplicate, out-of-order, and non-adjacent stages before
running the invalid stage. This keeps later compiler passes from silently
bypassing an earlier validation boundary.

## Stage Contract

Each stage is a function boundary:

```text
fn(&mut Context, &mut dyn ReportSink) -> StageOutcome
```

The context type is owned by the caller. The core pipeline does not allocate,
parse BASIC, lower IR, or emit target bytes.

## Error Propagation

A stage returns `StageOutcome::Failed(Diagnostic)` to stop the pipeline.
`run_pipeline` reports the diagnostic to the sink, emits a stage-finished
event, and returns `PipelineError::StageFailed`.

## Report Sink

`ReportSink` receives:

- stage-start events;
- stage-finished events;
- diagnostics emitted by failed stages.

`NullReportSink` is available for callers that do not need reports.

## IR Boundaries

`elderheim-ir` exposes boundary markers for:

- HIR to MIR;
- MIR to LIR;
- LIR to target.

These markers map back to the shared `PipelineStage` values so frontend and
backend crates use the same boundary vocabulary.

## Verification

The `0.4.0` stop requires:

- empty pipeline tests;
- stage ordering tests;
- missing-stage tests;
- error propagation tests;
- IR boundary mapping tests.
