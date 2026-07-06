# Changelog

All notable Elderheim changes are documented here.

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
