# Elderheim Supply Chain Security

Elderheim starts with zero third-party crate dependencies.

Policy:

- Admit dependencies only after a documented review.
- Prefer no_std-compatible crates when a dependency is unavoidable.
- Keep generated Dartmouth BASIC programs free of external compiler, linker,
  runtime, libc, LLVM, Cranelift, and BASIC runtime dependencies.
- Run `cargo deny check` and `cargo audit` through `scripts/checks.sh`.
- CI installs `cargo-deny` `0.20.2` and `cargo-audit` `0.22.2` before running
  `scripts/checks.sh`.
- Use release metadata checks and SBOM generation before release-candidate
  commits.

## SBOM Integrity

`scripts/generate-sbom.sh` generates SPDX 2.3 into temporary files and never
writes unvalidated generator output over the committed SBOM. The current
`cargo-sbom` `0.10.0` release emits one duplicate, byte-identical workspace-root
file record when a package has multiple targets. Elderheim rejects conflicting
duplicates and deterministically collapses only identical file records.

CI and local release environments must provide `jq`; CI verifies its presence
before running the gates. `scripts/validate-sbom.sh` verifies:

- SPDX 2.3 document identity and data license;
- uniqueness of every `SPDXID` in the document;
- that relationship source and target identifiers resolve to document
  elements, except standard `NONE` and `NOASSERTION` targets.

The normal check, release-candidate, and release-readiness gates validate the
committed SBOM. `scripts/test-sbom-validation.sh` injects a duplicate identifier
and proves the validator rejects it.
