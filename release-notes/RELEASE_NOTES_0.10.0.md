# Elderheim 0.10.0 Release Notes

Status: in development

## Scope

`0.10.0` starts the Dartmouth BASIC 1 frontend implementation line with a
stable line-table layer. It represents numbered BASIC 1 source before later
lexer and parser stages.

The release tag for this milestone is `elderheim-v0.10.0` because the plain
`v0.10.0` tag is unavailable in the repository.

This release does not add a BASIC lexer, full parser, semantic validator, MIR
lowering, runtime execution, or executable output.

## Added

- `Basic1LineNumber` for checked one-to-five digit line numbers.
- `Basic1LineTable` and `LineTableEntry` for ordered numbered source lines.
- `parse_basic1_line_table` as the public BASIC 1 line-table entry point.
- Stable line-table error kinds for malformed source shape.
- Duplicate line-number rejection.
- Strictly increasing line-number enforcement for BASIC 1 source order.
- Empty numbered line rejection.
- Missing statement-separator rejection.
- Tests for valid example line tables and malformed line-number cases.
- Project licensing/output-ownership documentation.

## Changed

- The Dartmouth BASIC crate is split into `dialect`, `line_table`, and `corpus`
  modules.
- The BASIC 1 corpus validator now consumes the line-table parser instead of
  carrying its own line-number ordering logic.
- README documentation now clarifies that generated user outputs are not
  automatically relicensed as Elderheim code.

## Security

- Malformed line-number handling fails closed before statement-level parsing.
- Duplicate and out-of-order source lines are rejected deterministically.
- Empty numbered lines cannot silently pass into later compiler stages.
- The generated-output licensing boundary is documented before runtime material
  is embedded into produced executables.

## Scope Exclusions

- BASIC 1 lexing starts in `v0.11.0`.
- BASIC 1 HIR shape starts in `v0.12.0`.
- BASIC 1 minimal parser fixtures start in `v0.13.0`.
