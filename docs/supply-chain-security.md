# Elderheim Supply Chain Security

Elderheim starts with zero third-party crate dependencies.

Policy:

- Admit dependencies only after a documented review.
- Prefer no_std-compatible crates when a dependency is unavoidable.
- Keep generated Dartmouth BASIC programs free of external compiler, linker,
  runtime, libc, LLVM, Cranelift, and BASIC runtime dependencies.
- Run `cargo deny check` and `cargo audit` through `scripts/checks.sh`.
- CI installs `cargo-deny` `0.19.9` and `cargo-audit` `0.22.2` before running
  `scripts/checks.sh`.
- Use release metadata checks and SBOM generation before release-candidate
  commits.
