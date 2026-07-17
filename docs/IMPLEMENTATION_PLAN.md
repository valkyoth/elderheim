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
  -> normalized source capability
  -> dialect lexer and CST
  -> dialect AST
  -> validated dialect semantic HIR
  -> validated target-neutral Elderheim MIR
  -> validated target-parametric LIR
  -> runtime fragment selection
  -> validated target instruction encoding
  -> checked relocation resolution
  -> validated executable image plan
  -> independent executable reparse and verification
  -> standalone executable
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
- integer expressions
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
- `print_i64`
- `read_line`
- `parse_i64`
- `bounds_fail`
- `div_zero_fail`

Every fragment has a declarative manifest covering required and provided
symbols, transitive dependencies, target services, code/data/scratch bounds,
register clobbers, calling convention, stack use, failure behavior, return
behavior, and accessible memory regions. Selection rejects cycles, missing
providers, incompatible targets, and unused fragments. Runtime symbols are
reserved and unforgeable by source programs.

## 8. Executable Output

1.0 output targets:

- Linux `x86` ELF32
- Linux `x86_64` ELF64
- Linux `aarch32` ELF32
- Linux `aarch64` ELF64
- Windows `x86_64` PE64
- macOS Apple Silicon `aarch64` Mach-O 64
- direct `_start`
- direct `write`, `read`, and `exit` syscalls as needed
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

Instruction encoders use sealed architecture/mode types, width-specific
registers and operands, instruction-specific constructors, checked relocation
records, and atomic emission into bounded sinks. No production API accepts raw
opcodes, arbitrary bytes, or untyped patch offsets. Exact manual-derived
vectors and an independently implemented decoder cover the emitted subset.

## 9. Reports

Reports are part of the product, not a side feature.

Planned reports:

- dialect report
- token and parse summary
- control-flow graph
- unreachable line report
- runtime-fragment inventory
- generated-binary dependency report
- syscall inventory
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

Production compiler, backend, writer, and runtime crates permanently use
crate-local `#![forbid(unsafe_code)]`. Policy gates reject `build.rs`, FFI,
native linking, inline assembly, production process spawning, and opaque native
runtime objects. Direct target service transitions emitted as validated machine
instructions are part of standalone output and are not external hooks.

Release gates must keep all non-generated Rust files below 500 lines.
