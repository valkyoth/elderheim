# Elderheim Implementation Plan

Status: planning document

Project name: `Elderheim`

Crate name: `elderheim`

Primary 1.0 target: prove the Elderheim compiler platform with a serious
production-ready Dartmouth BASIC implementation that can compile Dartmouth
BASIC versions 1, 2, and 4 source into standalone executables for Linux,
Windows, and macOS. Linux covers `x86`, `x86_64`, `aarch32`, and `aarch64`;
Windows covers `x86_64`; macOS covers Apple Silicon `aarch64`.

The 1.0 Dartmouth BASIC commitment is complete manual-backed source-language
support for versions 1, 2, and 4. It is not a partial subset plan. If a
committed item cannot be finished at the stop where it is discovered, the plan
must add a concrete follow-up release such as `0.1.1`, `0.2.1`, or the next
minor version with its own verification and pentest notes. Nothing in committed
scope may be left as an unnamed future task.

Dartmouth BASIC 4 means the documented programming language accepted in source
programs. Dartmouth timesharing session commands, editor commands, account/file
management commands, paper-tape workflows, and operating-system commands are
outside the compiler-language profile rather than deferred language features.

The generated executable path is strict:

- No Cranelift.
- No LLVM.
- No generated C or Rust.
- No system assembler.
- No system linker.
- No libc.
- No external BASIC runtime.
- No dynamic or static OS-library imports in generated programs.
- Direct machine-code emission.
- Direct executable-format writing.

The compiler implementation itself may use a thin `std` CLI for host file I/O.
Production compiler libraries should remain `no_std` by default.

## 1. Core Position

Elderheim is a preservation and sovereign-compilation platform. The first
production language target is:

```text
Dartmouth BASIC 1, 2, and 4
```

Other BASIC variants and older language families are future directions. They
must not get placeholder crates until source material, scope, and release
criteria are ready.

## 2. Workspace Architecture

```text
crates/elderheim
  facade library and host CLI shell

crates/elderheim-core
  spans, diagnostics, limits, IDs, byte sinks

crates/elderheim-digest
  no_std SHA-256 primitive, sealed domains, typed digests, bounded digest sinks

crates/elderheim-ir
  dialect-free MIR/LIR contracts, validators, bounded builders, lowering contracts

crates/elderheim-runtime
  runtime fragment inventory, dependency selection, inclusion reports

crates/elderheim-target
  target triples, ABI identifiers, executable-format choices

crates/elderheim-backend-x86
  x86 32-bit and 64-bit typed registers, instruction encoder, relocations

crates/elderheim-format-elf
  ELF32/ELF64 layout planner, explicit endian writer, image verifier

crates/languages/elderheim-dartmouth-basic
  Dartmouth BASIC versions 1, 2, and 4 CST, AST, semantic HIR, and profiles
```

## 3. Compiler Pipeline

```text
source bytes
  -> normalized source capability + normalized source digest
  -> dialect lexer and CST
  -> dialect AST
  -> validated dialect semantic HIR + opaque CompilationIdentity
  -> validated target-neutral Elderheim MIR
  -> validated TargetSpec + TargetServiceContractId
  -> derive runtime requirements
  -> validated RuntimePlan<Target>
  -> lower user program and runtime fragments into one bounded LIR builder
  -> validated target-parametric LIR
  -> validated arithmetic-guard and failure-lowering plan
  -> validated register/frame/machine-state plan preserving guard dominance
  -> symbolic target instruction regions and typed relocations
  -> bounded relaxation and provisional image layout planning
  -> assign file offsets and virtual addresses into ValidatedImageLayout
  -> checked relocation resolution and sealed regions
  -> validated whole-program ResourcePlan<Target>
  -> internal bounded SerializedImage staging buffer
  -> independent reparse plus final ResourceCertificate into VerifiedImage
  -> atomic user output publication
```

Every public frontend starts from a dialect-bound normalized-source capability;
raw `&str` helpers remain private. Spans use absolute offsets into that
normalized source. The token-bearing source representation is a CST, while
semantic HIR remains frontend-owned and dialect-aware. Shared IR starts at MIR.

The complete pipeline is encoded by capability types. A caller cannot omit
normalization, parsing, semantic validation, MIR validation, LIR validation,
encoding, image planning, or final image verification. Each backend or writer
accepts only the validated output type of its immediate predecessor. The
parser cannot emit CPU bytes, and operating-system services appear only through
target capability lowering and declared runtime fragments.

Runtime requirements are derived from validated MIR before LIR construction.
The selected `RuntimePlan<Target>` and user program lower into the same bounded
LIR builder, so final LIR validation covers both. Encoding produces symbolic
regions and typed relocations. Layout assigns file and virtual addresses before
relocations are resolved; relaxation and layout use a bounded monotonic
fixed-point process with a hard convergence limit.

All cryptographic identities use `elderheim-digest`: one frozen SHA-256
primitive, sealed versioned domains, and non-interchangeable output types. The
digest crate is below `elderheim-core` in the dependency graph so source
normalization can produce `SourceDigest` without a cycle. It has no knowledge
of Dartmouth rules, target contracts, resource plans, or executable formats.
Those owners assemble their own canonical preimages and independent validators
assemble/parse them separately; both may share the audited compression and
padding primitive without sharing domain-specific canonicalization logic.

`SemanticContractId` is a content fingerprint over the selected edition's
canonical rule tables, errata decisions, historical numeric model/constants,
RND behavior, and observable runtime contract. `CompilationIdentity` adds the
normalized source digest, frontend/MIR/LIR schema versions, relevant compiler
options/transforms, and complete effective limits. Untrusted decoded values
must first validate into sealed `SupportedSemanticContract` and
`ValidatedCompileConfiguration` capabilities; identity hashing consumes those
capabilities and never substitutes for support validation. Frontends construct
the opaque identity; shared MIR, runtime, backend, writer, and verifier code may
only preserve, compare, and report it. Every artifact boundary rejects an
identity mismatch, and shared IR never inspects dialect fields to choose
behavior. The normalized source digest is a separate domain-separated
cryptographic digest over normalized bytes, never the diagnostic `SourceId`.
The concrete outputs are typed `SourceDigest`, `SemanticFingerprint`, and
`CompilationFingerprint` values, not generic bytes or caller-selected
algorithms.

A transformation consumes one validated predecessor and builds a new candidate
transactionally. A fully revalidated semantics-preserving pass already listed
in the identity's transformation set retains that exact identity. Changing the
selected set or any bound source, semantics, schema, option, or limit creates a
new identity; failed, partial, or unlisted passes publish nothing and cannot
copy an identity through an unchecked path.

`TargetServiceContractId` versions architecture/mode, OS version range, service
ABI, entry, I/O, failure, termination, register, stack, memory, and error rules.
It combines a logical revision with a domain-separated fingerprint recomputed
from a deterministic canonical contract representation. Decoding produces an
untrusted parsed value; exact recognition and validation produce the sealed
service-contract capability required to construct the ID and target spec. The
same ID and fingerprint are bound into target specs, runtime plans, fragment
manifests, LIR, machine plans, resource certificates, verified images, and
compatibility evidence. Broad target-name, logical-revision, or matching-hash
equality never substitutes for supported-contract validation.
Target service validation uses only typed `TargetServiceFingerprint` values;
the target layer cannot choose a raw domain or algorithm.

The whole-program resource plan composes native frames/spills, runtime and
language control stacks, scratch and I/O buffers, arrays, DATA, writable state,
call depth, and mapped image memory before serialization. Independent image
verification supplies the final byte digest and produces `VerifiedImage` with
the finalized `ResourceCertificate`. Both bind the exact
`CompilationIdentity`. Missing, stale, cyclic/unbounded, or mismatched facts
prevent publication.

The certificate is metadata beside the executable bytes, not embedded within
them. Verification first produces typed `ResourcePlanDigest` and `ImageDigest`
values, then hashes the canonical certificate preimage containing those values,
the compilation identity, target-service fingerprint, and verifier version into
`CertificateDigest`. `VerifiedImage` carries these non-interchangeable fields
and has no certificate-free constructor, avoiding a digest cycle.

`SerializedImage` is internal staging data and cannot reach the CLI/filesystem
adapter. Only `VerifiedImage` can be published. Verification failure discards
the staging buffer and leaves any existing requested output unchanged.
Serialized executable bytes, reports, fingerprints, plans, and certificates
are untrusted inputs when read back. They cannot reconstruct a capability;
re-verification must parse and validate every layer, recompute every identity,
and match supported semantic, compile-configuration, and target-service
contracts. Certificates establish consistency, not producer authenticity.

## 4. Compiler-First Strategy

Elderheim should start with compiler infrastructure, not with a pile of BASIC
special cases. The first implementation passes build:

- source normalization;
- diagnostics and reports;
- HIR/MIR/LIR contracts;
- runtime fragment inventory;
- target and executable-format identifiers.

That compiler substrate must still be proven through a concrete language slice.
BASIC 1 is the first proving language because it forces real decisions about
line numbers, parsing, diagnostics, lowering, runtime output, and generated
executables without pulling in later-version complexity too early.

The rule is:

```text
compiler substrate -> BASIC 1 complete -> BASIC 2 complete -> BASIC 4 complete
```

BASIC 2 and BASIC 4 must be compatibility expansions with explicit deltas from
the previous version. They must not silently change BASIC 1 behavior.

## 5. Dartmouth BASIC Version Strategy

Dartmouth BASIC must be treated as documented versions, not a vague BASIC
grammar. Version 3 is intentionally out of scope until official documentation is
available.

Version profiles:

- `dartmouth-basic-1`
- `dartmouth-basic-2`
- `dartmouth-basic-4`

Each profile defines line-number rules, statement set, expression rules,
numeric model, arrays, `DATA` behavior, and diagnostics for constructs outside
that selected profile.

Profiles are sealed edition values backed by centralized, manual-derived rule
tables. Callers cannot construct arbitrary mixtures of edition behavior.
Shared grammar machinery handles common syntax, while genuinely different
edition rules may use separate implementations selected by the sealed profile.

Every implemented language profile must also have an Elderheim-authored
reference under `docs/languages/` and runnable source examples under
`examples/<profile>/`. Examples are part of the test corpus and must be
validated by automated tests before the profile can advance.

Implementation order:

1. `dartmouth-basic-1`
2. `dartmouth-basic-2`
3. `dartmouth-basic-4`

Version 3 is not planned until official documentation exists.

## 6. Dartmouth BASIC Manuals

The first manuals are local reference inputs:

- `/home/eldryoth/Work/test/basicmanuals/first edition may 1964.pdf`
- `/home/eldryoth/Work/test/basicmanuals/second edition october 1964.pdf`
- `/home/eldryoth/Work/test/basicmanuals/196801_BASIC_4th_Edition_text.pdf`

The first production command target is:

```bash
elderheim --dartmouth-basic-1 code.bas -o code
```

Each version grows to completion before the next version begins. Within a
version, semantic support generally grows in this order, with every feature
checked against the relevant manual:

- `PRINT` string literals and `END`
- `LET`
- historical numeric expressions
- `GOTO`
- `IF THEN`
- `FOR` / `NEXT`
- `GOSUB` / `RETURN`
- `INPUT`
- arrays
- `DATA` / `READ` / `RESTORE`

## 7. IR Rules

MIR is dialect-free and target-neutral. It uses explicit basic blocks, typed
block parameters and values, declared data objects and call signatures, and
one mandatory terminator per block. Validation proceeds deterministically
through shape, definitions, references and types, CFG and dominance, runtime
capabilities, and reachable-exit phases. It proves definite assignment and
rejects use-before-definition, malformed data references, unterminated blocks,
and capability mismatches.

The Dartmouth frontend owns CST, AST, and semantic HIR. Its validated semantic
HIR lowers transactionally into a bounded MIR builder. Failed lowering cannot
publish partial IR. Dense IDs and compilation-wide budgets permit predictable
linear validation where possible; bounded sort/search remains available for
adversarial raw IDs.

LIR is target-parametric and target-near. It represents typed machine-level
operations plus target-neutral service requests such as write, read, and
terminate until a validated target capability lowers them. It does not expose
generic Unix syscall operations or raw opcode bytes. A validated LIR for one
target cannot enter another target's backend.

Runtime helpers are fragments selected by use:

- `write_static`
- `print_number`
- `read_line`
- `parse_number`
- `bounds_fail`
- `div_zero_fail`

Every fragment has a declarative manifest covering required and provided
symbols, transitive dependencies, target services, code/data/scratch bounds,
register clobbers, calling convention, stack use, failure behavior, return
behavior, and accessible memory regions. Selection rejects cycles, missing
providers, incompatible targets, and unused fragments. Runtime symbols are
reserved and unforgeable by source programs.
Each manifest names one exact `TargetServiceContractId`; fragments are not
portable across service-contract revisions merely because target names match.

## 8. Executable Output

1.0 output targets:

- Linux `x86` ELF32
- Linux `x86_64` ELF64
- Linux `aarch32` ELF32
- Linux `aarch64` ELF64
- Windows `x86_64` PE64
- macOS Apple Silicon `aarch64` Mach-O 64
- direct validated target-specific process entry
- direct validated target service transitions for output, input, and
  termination
- no generated-program libc dependency
- no generated-program external BASIC runtime dependency

Security profiles add separate R, R|X, R, and R|W segments plus non-executable
stack metadata where the target format supports it.

Executable writers consume only private, validated image plans built from
domain newtypes for file offsets, virtual addresses, sizes, and alignments.
Planning checks narrowing, alignment congruence, overlap, table arithmetic,
W^X, target/class agreement, output budgets, and instruction-boundary entry
points. Writers use explicit endian field operations, emit the exact planned
size, and are followed by an independent parser/verifier that compares emitted
bytes with the plan.

All 1.0 target profiles are little-endian. Field writers still carry explicit
format/endianness types so a future target cannot silently inherit the current
matrix's byte order.

Instruction encoders use sealed architecture/mode types, width-specific
registers and operands, instruction-specific constructors, checked relocation
records, and atomic emission into bounded sinks. No production API accepts raw
opcodes, arbitrary bytes, or untyped patch offsets. Exact manual-derived
vectors and an independently implemented decoder cover the emitted subset.

Backend preparation validates liveness, physical-register assignment or a
documented fixed-register strategy, spill slots, frame size, stack alignment,
caller/callee-saved state, condition flags, runtime clobbers, and per-target CPU
feature baselines before instruction construction. Every physical register use
must have a dominating definition.

MIR transformations are optional and minimal for 1.0. Every pass consumes
validated MIR, builds a new result transactionally, validates it again, and
proves before/after observable-trace agreement in the independent interpreter.
No pass mutates validated MIR in place. Constant folding uses the historical
numeric model, and dead-code handling preserves compatibility and unreachable
line reports.

## 9. Reports

Reports are part of the product, not a side feature.

Planned reports:

- dialect report
- semantic-contract and compilation-identity report
- token and parse summary
- control-flow graph
- unreachable line report
- runtime-fragment inventory
- generated-binary dependency report
- target service inventory
- whole-program resource certificate
- compatibility warnings
- version-specific unsupported-feature diagnostics

## 10. Testing Policy

Every layer gets tests:

- dialect lexer tests
- parser tests from manual examples
- MIR validation tests
- exact-byte/word encoder tests for every supported CPU backend
- relocation math tests
- executable-format header and segment tests
- generated binary smoke tests
- report golden tests
- cross-version rejection tests so BASIC 1 does not accidentally accept BASIC 2
  or BASIC 4 syntax

Malformed-input and boundary tests land with the feature they protect, not in
a late hardening-only phase. Each stop covers applicable byte mutations,
resource boundaries, deterministic diagnostics, allocation failures, malformed
IR, relocation endpoints, layout boundary perturbations, and cross-target
rejection. The late security phase reruns cumulative campaigns.

Independent oracles include a pure Rust Dartmouth semantic interpreter, MIR
trace interpreter, emitted-instruction-subset decoder/interpreter, and
executable image reparser. Manual fixtures carry edition, source identifier,
page or rule reference, expected result, and expected rejection mode.

The semantic oracle is complete for BASIC 1 by `v0.19.0` and expands with the
BASIC 2 and BASIC 4 semantic stops. Each encoder ships its independent decoder
with its first instruction subset, and each executable format extends the
independent image parser with its first writer. The late oracle milestone is a
cumulative agreement gate, not the first oracle implementation.

Every user-controlled algorithm has a documented worst-case bound. Expression
parsing, name/line resolution, CFG and dominance analysis, runtime dependency
closure, register allocation, branch relaxation, interval layout, and
independent verification use bounded worklists and iteration caps. No
unbounded fixed point is permitted, and quadratic behavior is rejected unless
the input cap makes the measured worst case explicitly acceptable.

Production compiler, backend, writer, and runtime crates permanently use
crate-local `#![forbid(unsafe_code)]`. Policy gates reject `build.rs`, FFI,
native linking, inline assembly, production process spawning, and opaque native
runtime objects. Direct target service transitions emitted as validated machine
instructions are part of standalone output and are not external hooks.

Release gates must keep all non-generated Rust files below 500 lines.
