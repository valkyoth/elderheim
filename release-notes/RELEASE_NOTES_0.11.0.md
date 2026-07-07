# Elderheim 0.11.0 Release Notes

Status: in development

## Scope

`0.11.0` adds the Dartmouth BASIC 1 lexer layer. It tokenizes BASIC 1 statement
text after the numbered-line table and before later parser/HIR stages.

This release does not add a full BASIC parser, semantic validator, MIR lowering,
runtime execution, or executable output.

## Added

- `Basic1TokenKind` for BASIC 1 keywords, built-in functions, identifiers,
  numbers, strings, operators, relation operators, and delimiters.
- `Basic1Token` with statement-relative byte spans.
- `lex_basic1_statement` as the public BASIC 1 statement lexer entry point.
- `lex_basic1_statement_with_limits` for explicit compile-limit enforcement.
- BASIC 1 keyword and built-in function classification.
- Scalar variable identifier validation.
- User function identifier validation for `FN` plus one uppercase letter.
- Numeric literal scanning for integer, decimal, and `E` notation forms.
- Quoted string literal scanning.
- Lexer errors for invalid identifiers, invalid numbers, unterminated strings,
  unknown characters, and span overflow.
- Exact-output lexer fixture tests.
- Lexing coverage for every committed Dartmouth BASIC 1 example statement.

## Security

- Unknown characters fail closed.
- Invalid identifier shapes are rejected before parser work.
- Malformed exponent notation is rejected before parser work.
- Unterminated string literals cannot pass into later compiler stages.
- Token spans are checked before being stored.
- Lexer output is bounded by `CompileLimits::max_tokens`.

## Scope Exclusions

- BASIC 1 HIR shape starts in `v0.12.0`.
- BASIC 1 minimal parser fixtures start in `v0.13.0`.
- BASIC 1 expression parsing starts in later BASIC 1 parser stops.
