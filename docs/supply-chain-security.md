# Elderheim Supply Chain Security

Elderheim starts with zero third-party crate dependencies.

Policy:

- Admit dependencies only after a documented review.
- Prefer no_std-compatible crates when a dependency is unavoidable.
- Keep generated Dartmouth BASIC programs free of external compiler, linker,
  runtime, libc, LLVM, Cranelift, and BASIC runtime dependencies.
- Use `cargo deny`, release metadata checks, and SBOM generation once release
  artifacts exist.
