# Runtime Fragments

Status: active 0.8.0 contract

`elderheim-runtime` owns the runtime fragment inventory and dependency
selection model. The crate is `no_std`, allocation-free, and does not emit
machine code, object files, executable images, or platform runtime bytes.

## Requirements

Runtime selection starts from high-level compiler requirements:

- `StaticTextOutput`
- `IntegerOutput`
- `IntegerInput`
- `ArrayBoundsChecks`
- `DivisionByZeroChecks`
- `ProgramExit`

These names describe what a lowered program needs, not how a target implements
it.

## Fragments

The 0.8.0 fragment inventory is:

- `write_static`
- `print_i64`
- `read_line`
- `parse_i64`
- `bounds_fail`
- `div_zero_fail`
- `exit`

Fragment names are stable report-facing identifiers. Later runtime and backend
stops may lower these fragments into target-near LIR and target instructions,
but this stop only records selection.

## Selection Rules

`select_runtime` maps requirements to a `RuntimePlan`:

- static text output selects `write_static`;
- integer output selects `write_static` and `print_i64`;
- integer input selects `read_line` and `parse_i64`;
- array bounds checks select `write_static`, `bounds_fail`, and `exit`;
- division-by-zero checks select `write_static`, `div_zero_fail`, and `exit`;
- program exit selects `exit`.

Duplicate requirements are idempotent because `FragmentSet` is a fixed bitset.

## Inclusion Report

`RuntimePlan::inclusion_report()` returns a `FragmentInclusionReport`.
Callers can query stable `FragmentInclusion` entries per fragment. This gives
later CLI/reporting work a deterministic inventory surface without allocating
or serializing anything in the runtime crate.

## Non-Emission Contract

`RuntimePlan::emits_executable_artifacts()` returns `false`. Any future API
that writes executable bytes, object data, relocations, syscalls, or platform
runtime code belongs to a later runtime/backend stop and must add its own
tests, documentation, and pentest scope.

## Verification

The `0.8.0` stop requires:

- empty runtime selection tests;
- direct requirement-to-fragment tests;
- transitive dependency tests;
- duplicate requirement idempotence tests;
- fragment inclusion report tests;
- stable fragment-name tests;
- explicit non-emission tests.
