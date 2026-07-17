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
- A dedicated safe-Rust `no_std` typed digest crate freezes SHA-256, sealed
  domains, checked streaming lengths, and non-interchangeable digest outputs
  before the first cryptographic identity milestone.
- No LLVM, Cranelift, assembler, linker, or transpiler backend dependency in
  the planned generated-program path.
- Generated programs have no dynamic or static OS-library imports. Only
  documented stable direct target service transitions are eligible; an
  infeasible target service contract blocks at `v0.13.8` rather than
  weakening this rule implicitly.
- Every supported target service contract has an exact revisioned identity
  bound through target selection, runtime planning, fragment selection, LIR,
  machine planning, executable verification, and compatibility evidence.
  Validation recomputes its content fingerprint from a canonical contract form
  so an unchanged logical revision cannot conceal changed service semantics.
- Human-facing compiler snapshots and reports must escape untrusted source
  control bytes before writing terminal, CI log, or report text. Escape syntax
  must be canonical and distinguish literal backslashes from generated escapes.
- Language parsers must fail closed on unsupported statement forms and reject
  trailing tokens. The `v0.13.1` boundary requires all public frontends to
  consume normalized-source capabilities and report absolute source spans.
- A content-bound semantic contract and compilation identity bind edition
  rules, errata, numeric/runtime behavior, normalized source, IR schemas,
  options, and effective limits through HIR, MIR, runtime, LIR, resource
  certification, executable verification, oracle evidence, and reports.
  Shared compiler layers preserve this identity opaquely and reject mismatches.
  Its normalized-source digest is cryptographic and separate from diagnostic
  `SourceId` misuse detection.
- Hashing follows validation: only sealed supported semantic/target contracts
  and validated schemas, transformation sets, options, and limits can construct
  identities. Well-formed self-consistent unsupported data remains untrusted.
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
- Runtime requirements and a validated target runtime plan precede combined
  user/runtime LIR construction and final LIR validation.
- Typed block-structured MIR and target-parametric LIR with transactional
  bounded builders.
- Closed targets, target capabilities, typed relocations, and no ad hoc patching
  or public raw-opcode emission.
- Layout assigns final file and virtual addresses before relocation resolution;
  every patch is bounded, non-overlapping, sentinel-checked, exact-once, and
  sealed before serialization.
- Executable writers that serialize fields explicitly from private validated
  plans and are checked by independent image reparsers.
- Serialized images remain private staging buffers. Only an independently
  verified image capability can reach atomic CLI/filesystem publication.
- A checked whole-program resource plan composes native frames, runtime calls,
  language control stacks, storage, buffers, and mapped image memory. Image
  verification binds the final resource certificate to the verified digest
  before publication.
- Resource certificates are metadata beside executable bytes. The executable
  digest is computed first and the certificate digest then binds that digest,
  the canonical resource plan, compilation identity, service-contract
  fingerprint, and verifier version without self-referential image hashing.
- Canonical semantic, target-service, resource-plan, image, and certificate
  encodings require independently derived checked-in known-answer vectors and
  strict decoders that do not reuse production canonicalization helpers.
- Independent encoders/verifiers may share the audited digest primitive but
  must independently assemble and parse canonical domain preimages; duplicating
  SHA-256 implementations is not treated as validator independence.
- Deserialized executable bytes, reports, fingerprints, plans, and certificates
  never reconstruct validation capabilities. Complete re-verification must
  parse, validate, recompute, and match recognized supported contracts.
- Identity hashes and resource certificates prove deterministic consistency,
  not producer authenticity. Any future artifact-signing feature requires a
  separate explicit scope decision and must not be conflated with loader
  signature metadata.
- Secure ELF, PE, and Mach-O profiles enforce position/load policy, no RWX
  output, and format-appropriate load hardening; tiny profiles are not 1.0
  production outputs.
- Declarative runtime-fragment manifests and fragment/service inventories in
  generated binary reports.
- Incremental malformed-input and boundary testing at every feature stop, plus
  cumulative independent-oracle, mutation, and small-state campaigns.
