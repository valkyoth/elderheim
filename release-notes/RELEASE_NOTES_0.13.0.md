# Elderheim 0.13.0 Release Notes

Status: in development

## Scope

`0.13.0` adds the first complete Dartmouth BASIC 1 parser slice. It accepts
blank and comma-separated quoted-label `PRINT` statements followed by exactly
one final operand-free `END` statement.

This release does not add numeric expression parsing, the remaining BASIC 1
statement families, semantic validation, MIR lowering, runtime execution, or
executable output. Each of those features has a dedicated implementation stop
in the release plan.

## Added

- `Basic1ParsedProgram` and `Basic1ParsedLine`.
- `Basic1ParsedStatement` for the accepted `PRINT` and `END` grammar.
- `Basic1PrintStatement` and span-bearing `Basic1PrintItem` values.
- `parse_basic1_program` and `parse_basic1_program_with_limits`.
- Typed parser errors with BASIC line numbers and statement-relative spans.
- Strict final `END`, no-statements-after-`END`, and no-operands-after-`END`
  validation.
- Blank and comma-separated quoted-label `PRINT` parsing.
- Malformed `PRINT` separator regression tests.
- `examples/dartmouth-basic-1/print-labels.bas` as a committed parser fixture.

## Tooling

- Updated the pinned stable Rust toolchain and workspace Rust version to
  `1.97.0`.
- Updated the immutable `actions/checkout` pin to the `v7.0.0` commit.
- Updated the CI `cargo-deny` pin to `0.20.2`; verified that `cargo-audit`
  `0.22.2` remains current.

## Security

- Unsupported statement families and numeric `PRINT` items fail closed.
- Leading, repeated, and trailing `PRINT` separators are rejected.
- Adjacent `PRINT` labels without a separator are rejected.
- Tokens cannot be interpreted as implicit same-line statement terminators.
- Frontend source, line, and token limits remain enforced before parsing.
- Parser errors retain the BASIC line number and statement-relative token span
  when syntax-level context is available.

## Next Scheduled Stops

- BASIC 1 variables, numeric literals, `LET`, and expression parsing are
  implemented in `v0.14.0`.
- BASIC 1 control-flow parsing is implemented in `v0.15.0`.
- BASIC 1 semantic validation starts in `v0.16.0`.
