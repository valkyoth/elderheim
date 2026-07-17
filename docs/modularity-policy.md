# Elderheim Modularity Policy

Elderheim must not become a monolithic compiler.

Rules:

- The `elderheim` crate is a facade and CLI shell, not the implementation home.
- Shared compiler contracts live directly under `crates/`.
- Cryptographic digest primitives, sealed domains, and typed digest outputs live
  only in the planned `crates/elderheim-digest`; domain-specific canonical
  preimage encoders remain in their owning crates.
- The active language crate is `crates/languages/elderheim-dartmouth-basic`.
- Dartmouth BASIC versions 1, 2, and 4 must keep version rules explicit.
- Future language-family crates may be added only when the project has source
  material and a release plan for them.
- Backends and executable-format writers must stay separate.
- Non-generated Rust files must stay under 500 lines.
- Files approaching 300 lines should be split before more features are added.
- Core crates are `no_std` unless a documented exception is approved.

The local gate is:

```bash
scripts/validate-modularity-policy.sh check
```
