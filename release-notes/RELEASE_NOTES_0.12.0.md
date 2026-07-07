# Elderheim 0.12.0 Release Notes

Status: in development

## Scope

`0.12.0` adds the Dartmouth BASIC 1 source-shaped HIR layer. It represents
numbered BASIC 1 programs after line-table construction and statement lexing,
but before full parser and semantic validation work.

This release does not add a full BASIC parser, semantic validator, MIR lowering,
runtime execution, or executable output.

## Added

- `Basic1HirProgram` for ordered BASIC 1 source programs.
- `Basic1HirLine` for numbered source lines.
- `Basic1HirStatement` and `Basic1HirStatementKind` for BASIC 1 statement
  families.
- `Basic1HirExpression` for token-sequence expression operands.
- `build_basic1_hir` and `build_basic1_hir_with_limits`.
- `render_basic1_hir_snapshot` for stable HIR debug/report snapshots.
- HIR construction tests.
- Stable HIR snapshot tests for committed BASIC 1 fixtures.
- HIR construction coverage for every committed Dartmouth BASIC 1 example.

## Security

- HIR construction reuses line-table and lexer limit enforcement.
- Unsupported statement starts fail closed before later parser work.
- HIR snapshots are deterministic for review and regression testing.

## Scope Exclusions

- BASIC 1 minimal parser fixtures start in `v0.13.0`.
- BASIC 1 variables, numbers, and expressions parse in `v0.14.0`.
- BASIC 1 semantic validation starts in `v0.16.0`.
