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
  HIR/MIR/LIR contracts, validators, lowering contracts

crates/elderheim-target
  target triples, ABI identifiers, executable-format choices

crates/elderheim-backend-x86
  x86 32-bit and 64-bit typed registers, instruction encoder, relocations

crates/elderheim-format-elf
  ELF32/ELF64 layout planner, explicit endian writer, image verifier

crates/languages/elderheim-dartmouth-basic
  Dartmouth BASIC versions 1, 2, and 4
```

## 3. Compiler Pipeline

```text
source bytes
  -> source decoder
  -> dialect lexer
  -> dialect parser
  -> language HIR
  -> checked Elderheim MIR
  -> target-near LIR
  -> runtime fragment selection
  -> target instruction encoding
  -> relocation resolution
  -> executable image planning
  -> executable verification
  -> standalone executable
```

The parser must not emit CPU bytes directly. It emits source-shaped HIR.
Operating-system calls appear only in target lowering and runtime fragments.

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

MIR stays target-neutral:

- `WriteStatic`
- `WriteI64`
- `ReadI64`
- `Jump`
- `BranchIf`
- `CallLine`
- `Return`
- `Exit`

LIR is target-near:

- register moves
- RIP-relative data references
- branches
- calls
- Linux syscall lowering

Runtime helpers are fragments selected by use:

- `write_static`
- `print_i64`
- `read_line`
- `parse_i64`
- `bounds_fail`
- `div_zero_fail`

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

Release gates must keep all non-generated Rust files below 500 lines.
