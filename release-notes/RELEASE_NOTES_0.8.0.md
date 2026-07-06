# Elderheim 0.8.0 Release Notes

Status: released

## Scope

`0.8.0` completes the runtime fragment selection stop. It adds the
allocation-free runtime requirement and fragment inventory model used by later
runtime/backend work. It does not add executable emission, runtime byte output,
target lowering, BASIC parsing, or BASIC execution behavior.

## Added

- `elderheim-runtime` no_std workspace crate.
- Runtime requirement identifiers for output, input, failure paths, and exit.
- Runtime fragment identifiers for `write_static`, `print_i64`, `read_line`,
  `parse_i64`, `bounds_fail`, `div_zero_fail`, and `exit`.
- `FragmentSet` fixed-shape generated inventory set for allocation-free
  selection.
- `RuntimePlan` with an explicit non-emission contract.
- `select_runtime` requirement-to-fragment dependency selection.
- `FragmentInclusionReport` for stable report-facing fragment inclusion
  checks.
- Runtime fragment contract documentation under `docs/runtime-fragments.md`.

## Security

- Added tests proving empty selection, direct dependencies, transitive failure
  dependencies, duplicate requirement idempotence, stable fragment names, and
  non-emission behavior.
- Added no-unused-fragment and inclusion-report tests.
- Generated the runtime fragment inventory from one macro source to prevent
  enum/report drift.
- Removed hand-assigned per-fragment bit literals from the runtime fragment
  inventory.

## Scope Exclusions

- Runtime fragments do not lower into MIR or LIR yet.
- No target syscalls, ABI lowering, object bytes, executable images, or
  generated runtime blobs are emitted in this stop.
- BASIC-specific runtime requirements remain planned for later language stops.
