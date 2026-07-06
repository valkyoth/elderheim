# Changelog

All notable Elderheim changes are documented here.

## 0.5.0 - Unreleased

- Added no_std source normalization over caller-provided sinks.
- Added LF, CRLF, and CR line-ending normalization to LF.
- Added strict printable-ASCII/control-byte policy for current source input.
- Added blank-line policy with strict BASIC rejection and preserve modes.
- Added stable normalized source IDs.
- Split source handling into location and normalization modules.
- Refreshed the project overview image.
- Added `0.5.0` release notes and pentest draft.

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
