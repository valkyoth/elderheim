# Elderheim Security Controls

Initial controls:

- EUPL-1.2 licensing.
- Rust stable pin.
- Workspace lint policy currently forbids unsafe Rust. The `v0.13.2` boundary
  adds permanent crate-local `#![forbid(unsafe_code)]` to production compiler,
  frontend, IR, runtime, backend, and executable-writer crates.
- Production policy rejects build scripts, FFI, native linking, inline
  assembly, opaque native objects, and process-spawned toolchain stages.
- No external crates in `v0.1.0`.
- Explicit modularity policy with a 500-line hard source-file limit.
- No LLVM, Cranelift, assembler, linker, or transpiler backend dependency in
  the planned generated-program path.
- Human-facing compiler snapshots and reports must escape untrusted source
  control bytes before writing terminal, CI log, or report text. Escape syntax
  must be canonical and distinguish literal backslashes from generated escapes.
- Language parsers must fail closed on unsupported statement forms and reject
  trailing tokens. The `v0.13.1` boundary requires all public frontends to
  consume normalized-source capabilities and report absolute source spans.
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
- Mandatory typed pipeline capabilities; no stage can be skipped or resumed
  after failure.
- Typed block-structured MIR and target-parametric LIR with transactional
  bounded builders.
- Closed targets, target capabilities, typed relocations, and no ad hoc patching
  or public raw-opcode emission.
- Executable writers that serialize fields explicitly from private validated
  plans and are checked by independent image reparsers.
- Segment verification with no RWX output.
- Declarative runtime-fragment manifests and fragment/service inventories in
  generated binary reports.
- Incremental malformed-input and boundary testing at every feature stop, plus
  cumulative independent-oracle, mutation, and small-state campaigns.
