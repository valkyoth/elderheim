# Elderheim Security Controls

Initial controls:

- EUPL-1.2 licensing.
- Rust stable pin.
- Workspace lint policy forbids unsafe code, unchecked unsafe operations,
  `unwrap`, `expect`, panic, unchecked indexing, and undocumented unsafe blocks.
- No external crates in `v0.1.0`.
- Explicit modularity policy with a 500-line hard source-file limit.
- No LLVM, Cranelift, assembler, linker, or transpiler backend dependency in
  the planned generated-program path.
- 1.0 target matrix covers Linux `x86`, `x86_64`, `aarch32`, and `aarch64`,
  Windows `x86_64`, and macOS Apple Silicon `aarch64` without
  generated-program libc dependencies.

Planned compiler-output controls:

- Checked arithmetic for file offsets and relative displacements.
- Internal relocation table rather than ad hoc patching.
- ELF64 writer that serializes fields explicitly.
- Segment verifier with no RWX output in secure profiles.
- Runtime-fragment inventory in generated binary reports.
