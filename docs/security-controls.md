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
- Human-facing compiler snapshots and reports must escape untrusted source
  control bytes before writing terminal, CI log, or report text.
- Language parsers must fail closed on unsupported statement forms, reject
  trailing tokens, and attach statement-relative spans to syntax errors.
- MIR and LIR validation must remain subquadratic and bounded by operation
  limits under adversarial ID layouts.
- Executable layout validation must keep file offsets and virtual addresses in
  separate domains and reject arithmetic overflow.
- CLI arguments must be accepted as operating-system strings and rejected
  explicitly if a command requires Unicode text.
- Committed SPDX SBOMs must have unique element identifiers and valid
  relationship references before publication.
- 1.0 target matrix covers Linux `x86`, `x86_64`, `aarch32`, and `aarch64`,
  Windows `x86_64`, and macOS Apple Silicon `aarch64` without
  generated-program libc dependencies.

Planned compiler-output controls:

- Checked arithmetic for file offsets, virtual addresses, and relative
  displacements.
- Internal relocation table rather than ad hoc patching.
- ELF64 writer that serializes fields explicitly.
- Segment verifier with no RWX output in secure profiles.
- Runtime-fragment inventory in generated binary reports.
