# Elderheim Unsafe Policy

Elderheim production compiler, frontend, IR, runtime, backend, and
executable-writer crates must permanently forbid unsafe Rust. The workspace
lint enforces that rule today. The `v0.13.2` boundary adds crate-local
`#![forbid(unsafe_code)]` so workspace configuration is an additional gate, not
the only prohibition.

Production rules:

- Prefer safe Rust and explicit byte serialization.
- Do not cast ELF, PE, or Mach-O structs into byte slices.
- Do not add `build.rs`, FFI declarations, `#[link]`, inline assembly, native
  libraries, opaque object archives, or production `std::process` execution.
- Do not use external assemblers, linkers, C libraries, runtime hooks, static
  hooks, or operating-system tool binaries in the supported generated-program
  path.
- Serialize machine instructions and executable fields through checked safe
  Rust APIs and bounded byte sinks.
- Independently decode or reparse generated instructions and executable images
  before publication.

Generated programs contain machine instructions, including validated direct
target service transitions needed for standalone executables. Those
instructions are output data produced by safe Rust; they are not Rust unsafe
blocks, external hooks, FFI, or native compiler dependencies.

Any future proposal that cannot satisfy this policy requires an explicit
project-scope and release-plan decision before implementation. It cannot be
introduced as a local exception to an existing production crate.
