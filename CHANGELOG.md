# Changelog

All notable Elderheim changes are documented here.

## 0.11.0 - 2026-07-07

- Added the no_std Dartmouth BASIC 1 statement lexer.
- Added BASIC 1 keyword, built-in function, identifier, number, string,
  operator, relation, and delimiter token kinds.
- Added statement-relative token span storage.
- Added invalid identifier, invalid number, unterminated string, unknown
  character, and span-overflow lexer errors.
- Added `CompileLimits::max_tokens` enforcement to the lexer.
- Added exact-output lexer fixture tests and committed BASIC 1 example lexing
  coverage.
- Updated Dartmouth BASIC 1 documentation with the lexer policy.
- Added `0.11.0` release notes and pentest draft.

## 0.10.0 - 2026-07-06

- Added the no_std Dartmouth BASIC 1 line-number parser.
- Added the BASIC 1 line-table model and public parser entry point.
- Added duplicate, malformed, out-of-order, missing-separator, and empty
  numbered line diagnostics.
- Enforced compile byte and line limits in the BASIC 1 line-table parser.
- Replaced duplicate line-number scanning with tree-backed duplicate detection.
- Wired the BASIC 1 corpus validator through the line-table parser.
- Added line-table coverage for every committed BASIC 1 example.
- Split the Dartmouth BASIC crate into focused modules.
- Added project licensing/output-ownership documentation.
- Added `0.10.0` release notes and pentest draft.

## 0.9.0 - 2026-07-06

- Added `docs/languages/` for Elderheim-authored language references.
- Added the Dartmouth BASIC 1 corpus reference.
- Added the manual corpus policy and local manual provenance document.
- Added committed Dartmouth BASIC 1 examples and a fixture manifest.
- Added BASIC 1 corpus validation tests in the Dartmouth BASIC crate.
- Added manual corpus validation to the normal local check gate.
- Added `0.9.0` release notes and pentest draft.
- Addressed the `0.9.0` pentest finding around `PRINT` keyword-boundary
  validation.

## 0.8.0 - 2026-07-06

- Added the `elderheim-runtime` no_std crate.
- Added runtime requirement identifiers for output, input, failure paths, and
  program exit.
- Added runtime fragment identifiers for static output, integer formatting,
  line input, integer parsing, bounds failure, division failure, and exit.
- Added allocation-free runtime fragment dependency selection.
- Added an explicit non-emission contract for runtime plans.
- Added runtime fragment inclusion reports and no-unused-fragment tests.
- Generated the runtime fragment inventory from one macro source to prevent
  enum/report drift.
- Removed hand-assigned per-fragment bit literals from the runtime fragment
  inventory.
- Added runtime fragment documentation, release notes, and pentest draft.
- Addressed the `0.8.0` pentest findings around runtime fragment inventory
  drift and the retest note around manual bit assignment.

## 0.7.0 - 2026-07-06

- Added HIR node and symbol ID contracts.
- Added MIR value and label ID contracts.
- Added LIR label and symbol ID contracts.
- Added no_std HIR, MIR, and LIR validation entry points.
- Added validated HIR, MIR, and LIR wrapper types.
- Added MIR and LIR program size caps for bounded validation.
- Added version-neutral lowering sink traits for HIR-to-MIR and MIR-to-LIR.
- Required validated IR at lowering trait boundaries.
- Added IR error-code uniqueness regression coverage.
- Split the IR crate into focused modules.
- Added `0.7.0` release notes and pentest draft.
- Addressed the `0.7.0` pentest findings around bounded MIR/LIR validation,
  validated lowering boundaries, and IR diagnostic-code uniqueness.

## 0.6.0 - 2026-07-06

- Added a stable diagnostic code registry.
- Added source snippet diagnostic rendering.
- Added visible malformed-snippet failure handling.
- Added report sections and report events for pipeline and diagnostic reports.
- Added golden tests for diagnostic snippets and report rendering.
- Split diagnostic tests out of the production diagnostic module.
- Addressed the `0.6.0` pentest findings around snippet rendering complexity,
  cursor source binding, diagnostic source binding semantics, and diagnostic
  registry completeness.

## 0.5.0 - 2026-07-06

- Added no_std source normalization over caller-provided sinks.
- Added LF, CRLF, and CR line-ending normalization to LF.
- Added strict printable-ASCII/control-byte policy for current source input.
- Added blank-line policy with strict BASIC rejection and preserve modes.
- Added stable normalized source IDs.
- Added public normalized-source construction that rejects unnormalized bytes.
- Centralized normalized byte and blank-line policy in one shared scanner.
- Added visible diagnostic-location failure rendering.
- Split source handling into location and normalization modules.
- Refreshed the project overview image.
- Addressed the `0.5.0` pentest findings around raw source construction,
  diagnostic location failures, sink error contracts, `SourceId` trust
  boundaries, and duplicated source policy state machines.

## 0.4.0 - 2026-07-06

- Added the no_std compiler pipeline skeleton.
- Added ordered source-to-diagnostic, HIR-to-MIR, MIR-to-LIR, and
  LIR-to-target stage contracts.
- Added report sink events for stage start, finish, and diagnostics.
- Added pipeline tests for empty pipelines, stage ordering, missing stages,
  and error propagation.
- Added IR boundary markers mapped to shared pipeline stages.
- Updated locked milestone tag guidance to use version-suffixed tags.
- Addressed the `0.4.0` pentest findings around skipped pipeline stages,
  release tag validation, supply-chain gate enforcement, and module size.

## 0.3.0 - 2026-07-06

- Added the no_std source byte model.
- Added checked spans, one-based line/column lookup, and cursor-based lookup
  for batched diagnostics.
- Added stable diagnostic codes, severity labels, and compact diagnostic
  rendering.
- Added source byte and line limit enforcement tests.
- Added source/diagnostic contract documentation.
- Addressed the `0.3.0` pentest findings around diagnostic lookup complexity,
  source validation ordering, release-readiness temp-file handling, and span
  invariants.

## 0.2.0 - 2026-07-06

- Added canonical CLI-visible names for every supported 1.0 target.
- Added no_std target-name parsing and stable rejection diagnostics.
- Added CLI target listing and target validation scaffolding.
- Added `0.2.0` release notes and pentest draft.

## 0.1.0 - 2026-07-06

- Initialized the Elderheim Rust workspace.
- Added EUPL-1.2 licensing and security policy scaffolding.
- Added no_std core, IR, target, backend, ELF64, and language crate skeletons.
- Added implementation and release plans through the BASIC-family 1.0 target.
- Added local verification scripts.
