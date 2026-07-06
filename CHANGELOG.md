# Changelog

All notable Elderheim changes are documented here.

## 0.8.0 - Unreleased

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
- Added runtime fragment documentation, release notes, and pentest draft.

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
- Updated locked milestone tag guidance to use `elderheim-vX.Y.Z`.
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
