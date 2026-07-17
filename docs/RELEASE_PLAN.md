# Elderheim Release Plan To 1.0

Status: planning document

This plan is intentionally granular. Elderheim is not a small parser project:
it is a direct native compiler, executable writer, runtime-fragment system, and
compatibility project for Dartmouth BASIC versions 1, 2, and 4.

`v1.0.0` is the first serious production-ready language milestone for the
Elderheim compiler platform. It proves the platform with Dartmouth BASIC. It is
not the end of Elderheim.

The 1.0 Dartmouth BASIC commitment is full manual-backed source-language
support for versions 1, 2, and 4. Release tags must not claim a profile,
target, report, or security control is done while carrying an unnamed partial
implementation. If work inside committed scope cannot finish at the current
stop, this plan must add a concrete follow-up version such as `0.1.1`,
`0.2.1`, or the next minor version with its own definition of done, tests,
release notes, and pentest notes.

Dartmouth BASIC 4 scope is the source programming language. Dartmouth
timesharing session commands, editor commands, account/file management,
paper-tape workflows, and operating-system commands are not part of the
compiler-language profile and are not treated as deferred BASIC 4 features.

## 1.0 Scope

`v1.0.0` must support:

- Dartmouth BASIC version 1.
- Dartmouth BASIC version 2.
- Dartmouth BASIC version 4.
- Dartmouth BASIC version 3 remains unsupported until official documentation is
  available.
- Standalone Linux ELF output for:
  - `x86` 32-bit, ELF32.
  - `x86_64` 64-bit, ELF64.
  - `aarch32` 32-bit ARM/AArch32, ELF32.
  - `aarch64` 64-bit ARM/AArch64, ELF64.
- Standalone Windows PE output for:
  - `x86_64` 64-bit, PE64.
- Standalone macOS Mach-O output for:
  - Apple Silicon `aarch64`, Mach-O 64-bit.
- Direct machine-code emission.
- Direct executable-format writing.
- No generated C or Rust.
- No Cranelift.
- No LLVM.
- No system assembler.
- No system linker.
- No libc dependency in generated programs.
- No external BASIC runtime dependency in generated programs.
- No dynamic or static OS-library imports in generated programs.
- Compatibility reports.
- Generated-binary dependency reports.
- Security and release evidence.

Windows support is intentionally scoped to `x86_64` for 1.0. macOS support is
intentionally scoped to Apple Silicon `aarch64` for 1.0.

## Version Sequencing Rule

Dartmouth BASIC versions must be implemented in order:

1. BASIC 1 reaches complete manual-backed source-language support.
2. BASIC 2 is added as a compatibility expansion on top of proven BASIC 1
   infrastructure.
3. BASIC 4 is added only after BASIC 2 is complete.

The compiler substrate may be built before and during BASIC 1 work, but BASIC 2
and BASIC 4 features must not leak into BASIC 1 acceptance tests. Cross-version
tests should prove that selecting an older version rejects later-version syntax
or semantics with stable diagnostics.

## Release Rules

Every release must have:

- A clear definition of done.
- Local verification commands.
- Release notes.
- Security review or pentest notes for the exact commit being tagged.
- A pentest report built from supplied `PENTEST.md` input when present.
- CodeQL default setup findings reviewed and reflected in the pentest report.
- No hidden dependency on a developer machine.
- Documentation of scope exclusions only for work explicitly outside the
  release scope, or a scheduled follow-up version for any incomplete committed
  scope.
- No source file over 500 lines.
- Malformed-input, boundary, budget, deterministic-diagnostic, and fail-closed
  tests for every parser, validator, runtime, encoder, relocation, or writer
  behavior added by that release.
- No IR, encoded program, or executable image may be published after an error;
  partial builder output remains inaccessible.
- Compiler-controlled allocations are budgeted before allocation and report a
  structured resource error where Rust allocation APIs permit recovery.
- Every user-controlled algorithm documents and tests its worst-case time,
  memory, worklist, recursion, and iteration bounds. No unbounded fixed point is
  permitted; quadratic behavior requires an explicit measured justification
  under the configured input cap.
- Independent semantic, decoder, and image-parser oracles arrive with the first
  production layer they verify and expand at every applicable stop. Late
  security milestones are cumulative agreement gates, not first validation.
- External emulators and native target execution may provide separately
  recorded compatibility evidence and may be required before claiming final
  target support, but cannot be compiler implementation/test dependencies.
  In-process instruction interpreters and image reparsers provide mandatory
  deterministic verification at implementation stops.

Local commits may be made regularly while work is progressing. Maintainers push
the branch and tags. Tags are release events and require the release procedure in
`docs/release-procedure.md`.

Tag readiness requires:

- `scripts/checks.sh` passes.
- `cargo deny check` passes.
- `cargo audit` passes.
- `scripts/generate-sbom.sh` succeeds.
- Release notes exist under `release-notes/`.
- A pentest report exists under `security/pentest/`.
- The pentest report includes CodeQL status and any findings or follow-up
  releases.
- Root `PENTEST.md` scratch input has been incorporated into the final report
  and removed.
- The tag does not already exist locally.

## Immutable Tag Workaround

The new repository already has immutable releases for these tag names:

- `v0.10.0`
- `v0.20.0`
- `v0.30.0`
- `v0.40.0`
- `v0.50.0`
- `v0.60.0`
- `v0.70.0`
- `v0.80.0`
- `v0.90.0`

Use `vX.Y.Z-elderheim` version-suffixed tags for those exact milestones:

- `v0.10.0-elderheim`
- `v0.20.0-elderheim`
- `v0.30.0-elderheim`
- `v0.40.0-elderheim`
- `v0.50.0-elderheim`
- `v0.60.0-elderheim`
- `v0.70.0-elderheim`
- `v0.80.0-elderheim`
- `v0.90.0-elderheim`

This workaround applies only to those exact `0.x.0` milestones. Other
pre-1.0 tags keep the normal `vX.Y.Z` form. `v1.0.0` has not been tagged and
keeps the normal final-release name.

## Stop Gates And Pentest Classes

Each version has two release controls:

- `Stop gate`: the exact point where implementation must stop and review starts.
- `Pentest class`: the security-review depth required before tagging.

The detailed milestone section gives the version-specific verification list.
The matrix below assigns the required stop and pentest class for every tag.

Pentest classes:

- `P0 - Foundation`: repository, policy, metadata, CI, and documentation.
- `P1 - Compiler Core`: source handling, diagnostics, IR contracts, reports,
  and non-executable compiler substrate.
- `P2 - Language Frontend`: lexer, parser, semantic validation, version gates,
  independent semantic oracle, and compatibility fixtures.
- `P3 - Runtime`: generated-program runtime fragments and runtime error paths.
- `P4 - Executable Format`: ELF, PE, Mach-O writers, layout, headers,
  permissions, and generated-binary dependency reports.
- `P5 - Native Backend`: instruction encoders, relocations, ABI lowering,
  native executable smoke tests, and target-specific runtime behavior.
- `P6 - Product Surface`: CLI, reports, compatibility matrices, target
  matrices, and user-facing diagnostics.
- `P7 - Release Security`: abuse hardening, reproducibility, release evidence,
  security review, pentest fixes, and final release acceptance.

| Version | Stop gate | Pentest class |
| --- | --- | --- |
| v0.1.0 | Repository foundation compiles and policy docs are present. | P0 |
| v0.2.0 | Full 1.0 target matrix is represented and unsupported targets fail. | P0 |
| v0.3.0 | Source and diagnostic core passes golden and limit tests. | P1 |
| v0.4.0 | Compiler pipeline boundaries pass ordering and error propagation tests. | P1 |
| v0.5.0 | Source normalization rejects invalid/oversized input deterministically. | P1 |
| v0.6.0 | Diagnostics and report rendering have stable golden output. | P1 |
| v0.7.0 | HIR/MIR/LIR contracts and validators are testable. | P1 |
| v0.8.0 | Runtime fragment model selects dependencies without emitting executables. | P1 |
| v0.9.0 | Manual fixture corpus is controlled and local-manual absence is handled. | P0 |
| v0.10.0-elderheim | BASIC 1 line table behavior is stable. | P2 |
| v0.11.0 | BASIC 1 lexer fixtures pass. | P2 |
| v0.12.0 | BASIC 1 HIR snapshots are stable. | P2 |
| v0.13.0 | BASIC 1 minimal parser fixtures pass. | P2 |
| v0.13.1 | Every frontend requires normalized source and reports absolute spans. | P2 |
| v0.13.2 | CST/semantic-HIR ownership and a mandatory complete pipeline are enforced by types. | P1 |
| v0.13.3 | Compilation budgets and transactional bounded builders are unified. | P1 |
| v0.13.4 | Dartmouth edition profiles and manual-backed semantic tables are sealed. | P2 |
| v0.13.5 | BASIC 1 historical numeric semantics are specified and executable in a reference model. | P2 |
| v0.13.6 | Manual provenance and two-way language-rule coverage ledgers are complete. | P2 |
| v0.13.7 | Versioned direct target-service contracts have complete no-import feasibility evidence. | P5 |
| v0.14.0 | BASIC 1 variables, numbers, and expressions parse correctly. | P2 |
| v0.15.0 | BASIC 1 control-flow parser rejects later-version syntax. | P2 |
| v0.16.0 | BASIC 1 semantic validation and CFG reports pass. | P2 |
| v0.16.1 | Typed block-structured MIR rejects CFG, dominance, type, data, and capability defects. | P1 |
| v0.16.2 | MIR construction is transactional, bounded, and independently stress-tested. | P1 |
| v0.16.3 | Frontend diagnostics carry stable stage codes, secondary spans, edition requirements, and manual rules. | P2 |
| v0.16.4 | Every MIR transformation is transactional, revalidated, bounded, and trace-equivalent. | P1 |
| v0.17.0 | BASIC 1 MIR golden tests pass. | P2 |
| v0.18.0 | BASIC 1 runtime requirements are fully inventoried. | P3 |
| v0.18.1 | Portable output, input, EOF, error, and exit semantics are frozen. | P3 |
| v0.18.2 | Historical numeric functions and failure behavior pass the independent reference model. | P2 |
| v0.18.3 | Edition-specific number parsing, formatting, and deterministic RND behavior pass. | P3 |
| v0.18.4 | Semantic contracts and compilation identities are content-bound and preserved end to end. | P2 |
| v0.19.0 | The independent BASIC 1 semantic oracle executes every semantic fixture. | P2 |
| v0.20.0-elderheim | BASIC 1 compatibility sweep passes before BASIC 2 starts. | P2 |
| v0.21.0 | BASIC 2 delta is documented and BASIC 1 remains green. | P2 |
| v0.22.0 | BASIC 2 lexer delta passes and BASIC 1 rejects BASIC 2-only tokens. | P2 |
| v0.23.0 | BASIC 2 parser delta passes without BASIC 1 regressions. | P2 |
| v0.24.0 | BASIC 2 semantic validation passes without BASIC 1 regressions. | P2 |
| v0.25.0 | BASIC 2 MIR/runtime delta passes without BASIC 1 regressions. | P2 |
| v0.26.0 | The independent semantic oracle executes BASIC 1 and BASIC 2 fixtures. | P2 |
| v0.27.0 | BASIC 2 compatibility sweep passes before BASIC 4 starts. | P2 |
| v0.28.0 | BASIC 4 delta is documented and older versions remain green. | P2 |
| v0.29.0 | BASIC 4 lexer delta passes and older modes reject BASIC 4-only tokens. | P2 |
| v0.30.0-elderheim | BASIC 4 parser delta passes without older-version regressions. | P2 |
| v0.31.0 | BASIC 4 semantic validation passes without older-version regressions. | P2 |
| v0.32.0 | BASIC 4 MIR/runtime delta passes without older-version regressions. | P2 |
| v0.33.0 | The independent semantic oracle executes BASIC 1, BASIC 2, and BASIC 4 fixtures. | P2 |
| v0.34.0 | BASIC 1, 2, and 4 compatibility sweeps all pass. | P2 |
| v0.34.1 | Closed target types are integrated with capabilities, service conventions, and ABIs. | P3 |
| v0.34.2 | LIR is target-parametric and rejects cross-target service lowering. | P3 |
| v0.34.3 | Runtime manifests prove dependencies, symbols, bounds, clobbers, and target compatibility. | P3 |
| v0.34.4 | A shared backend contract supports x86 and AArch without raw-byte or x86-shaped leakage. | P5 |
| v0.35.0 | A validated runtime plan and user program lower together into fully validated LIR. | P3 |
| v0.36.0 | Write and exit runtime behavior is platform-abstracted. | P3 |
| v0.37.0 | Historical-number formatting runtime passes bounds and golden tests. | P3 |
| v0.37.1 | Complete edition-specific PRINT statement layout emits exact byte streams. | P3 |
| v0.38.0 | Input runtime passes valid and invalid input tests. | P3 |
| v0.38.1 | Complete edition-specific INPUT statement traces match the semantic oracle. | P3 |
| v0.38.2 | Production historical arithmetic and comparison runtime matches the oracle. | P3 |
| v0.38.3 | Production numeric functions and deterministic RND runtime match the oracle. | P3 |
| v0.39.0 | DATA and array runtime passes bounds tests. | P3 |
| v0.39.1 | Executable image domain types and checked layout planning invariants pass. | P4 |
| v0.39.2 | Bounded writers and independent image reparsing reject plan/byte mismatches. | P4 |
| v0.39.3 | Runtime memory, service-loop, DATA, array, and control-stack safety passes. | P3 |
| v0.39.4 | Position independence, load-bias, image-base, and hardening policy is frozen. | P4 |
| v0.39.5 | Whole-program resource certificate contracts and composition validation pass. | P5 |
| v0.39.6 | Only resource-certified, independently verified images can be atomically published. | P5 |
| v0.40.0-elderheim | ELF writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.41.0 | ELF64 tiny profile is layout-verified. | P4 |
| v0.42.0 | ELF32 tiny profile is layout-verified. | P4 |
| v0.43.0 | Secure ELF64 profile enforces segment permissions. | P4 |
| v0.44.0 | Secure ELF32 profile enforces segment permissions and address bounds. | P4 |
| v0.44.1 | Typed arithmetic guards and failure lowering contracts precede machine planning. | P5 |
| v0.44.2 | Register allocation, frame layout, machine-state, ABI, and CPU-baseline validation pass. | P5 |
| v0.44.3 | Typed encoders, relocations, atomic emission, and independent decoding contracts pass. | P5 |
| v0.44.4 | Relaxation/layout converges before every relocation is resolved and sealed exactly once. | P5 |
| v0.45.0 | x86_64 encoder exact-byte tests pass. | P5 |
| v0.46.0 | x86_64 relocation boundary tests pass. | P5 |
| v0.46.1 | x86_64 arithmetic trap-equivalence traces match historical semantics. | P5 |
| v0.47.0 | x86_64 ELF64 in-process hello smoke passes; native evidence is separate. | P5 |
| v0.48.0 | x86_64 interpreter Dartmouth core suite passes; native evidence is separate. | P5 |
| v0.49.0 | x86 32-bit encoder exact-byte tests pass. | P5 |
| v0.50.0-elderheim | x86 32-bit relocation boundary tests pass. | P5 |
| v0.50.1 | x86 32-bit arithmetic trap-equivalence traces match historical semantics. | P5 |
| v0.51.0 | x86 ELF32 in-process hello smoke passes; native evidence is separate. | P5 |
| v0.52.0 | x86 interpreter Dartmouth core suite passes; native evidence is separate. | P5 |
| v0.53.0 | AArch64 encoder exact-word tests pass. | P5 |
| v0.54.0 | AArch64 relocation boundary tests pass. | P5 |
| v0.54.1 | AArch64 arithmetic trap-equivalence traces match historical semantics. | P5 |
| v0.55.0 | AArch64 ELF64 in-process hello smoke passes; native evidence is separate. | P5 |
| v0.56.0 | AArch64 interpreter Dartmouth core suite passes; native evidence is separate. | P5 |
| v0.57.0 | AArch32 encoder exact-word tests pass. | P5 |
| v0.58.0 | AArch32 relocation boundary tests pass. | P5 |
| v0.58.1 | AArch32 arithmetic trap-equivalence traces match historical semantics. | P5 |
| v0.59.0 | AArch32 ELF32 in-process hello smoke passes; native evidence is separate. | P5 |
| v0.60.0-elderheim | AArch32 interpreter Dartmouth core suite passes; native evidence is separate. | P5 |
| v0.61.0 | PE writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.62.0 | PE64 tiny profile is layout-verified. | P4 |
| v0.62.1 | Secure PE64 enables the approved image-base, relocation, NX, and ASLR policy. | P4 |
| v0.63.0 | Windows x86_64 ABI lowering tests pass. | P5 |
| v0.64.0 | Windows x86_64 PE in-process hello smoke passes; native evidence is separate. | P5 |
| v0.65.0 | Windows x86_64 interpreter Dartmouth suite passes; native evidence is separate. | P5 |
| v0.66.0 | Mach-O writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.67.0 | Mach-O AArch64 tiny profile is layout-verified. | P4 |
| v0.67.1 | Secure Mach-O AArch64 position-independent layout and load metadata pass. | P4 |
| v0.68.0 | macOS AArch64 ABI lowering tests pass. | P5 |
| v0.69.0 | macOS AArch64 Mach-O in-process hello smoke passes; native evidence is separate. | P5 |
| v0.70.0-elderheim | macOS AArch64 interpreter Dartmouth suite passes; native evidence is separate. | P5 |
| v0.71.0 | Cross-platform runtime conformance matches the frozen observable contract. | P6 |
| v0.72.0 | Cross-platform output matrix is represented and documented. | P6 |
| v0.72.1 | Final resource certificates pass for every language profile and target. | P7 |
| v0.73.0 | CLI target selection tests pass for every 1.0 target. | P6 |
| v0.74.0 | Dartmouth version CLI tests pass and BASIC 3 is rejected. | P6 |
| v0.75.0 | Compatibility report golden tests pass. | P6 |
| v0.76.0 | Generated-binary report golden tests pass. | P6 |
| v0.77.0 | User-facing diagnostic golden tests pass. | P6 |
| v0.78.0 | BASIC 1 compatibility sweep passes on implemented targets. | P2 |
| v0.79.0 | BASIC 2 compatibility sweep passes on implemented targets. | P2 |
| v0.80.0-elderheim | BASIC 4 compatibility sweep passes on implemented targets. | P2 |
| v0.81.0 | Cross-version compatibility matrix passes. | P6 |
| v0.82.0 | Cross-target output matrix passes. | P6 |
| v0.82.1 | Independent semantic, MIR, instruction-trace, and image oracles agree. | P7 |
| v0.82.2 | Deterministic mutation, property, and small-state model campaigns pass. | P7 |
| v0.83.0 | Cumulative source and parser abuse campaigns pass without panics. | P7 |
| v0.84.0 | Cumulative encoder, relocation, and layout abuse campaigns pass. | P7 |
| v0.85.0 | Cumulative runtime fragment abuse campaigns pass on all 1.0 targets. | P7 |
| v0.86.0 | Dependency, audit, SBOM, modularity, and release gates pass. | P7 |
| v0.87.0 | Reproducible generated-output tests pass. | P7 |
| v0.88.0 | Documentation freeze candidate has complete links and examples. | P7 |
| v0.89.0 | Feature freeze candidate has complete fixture and target matrices. | P7 |
| v0.90.0-elderheim | Performance and size baselines are recorded. | P7 |
| v0.91.0 | Security review handoff evidence is complete. | P7 |
| v0.92.0 | Pentest fixes have regression tests and follow-up review. | P7 |
| v0.93.0 | Final 1.0 candidate has no known release blockers. | P7 |
| v1.0.0 | Final release evidence, fixture matrix, target matrix, and pentest PASS are complete. | P7 |

## Phase 0: Foundation

### v0.1.0 - Repository Foundation

Goal:

Create the initial workspace, policy, documentation, and no_std crate skeleton.

Deliverables:

- Root `Cargo.toml`.
- `rust-toolchain.toml` pinned to the current stable Rust release.
- EUPL-1.2 `LICENSE`.
- `README.md`.
- `.github` metadata.
- `deny.toml`.
- Shared compiler crates under `crates/`.
- Dartmouth BASIC language crate under `crates/languages/`.
- `docs/IMPLEMENTATION_PLAN.md`.
- `docs/RELEASE_PLAN.md`.
- Unsafe, modularity, threat-model, supply-chain, and toolchain docs.
- Local check scripts.

Verification:

- `scripts/checks.sh`
- `cargo deny check`
- `cargo audit`
- `scripts/generate-sbom.sh`

Exit criteria:

- The workspace compiles.
- A contributor can see the intended compiler architecture.
- No production Rust source file exceeds 500 lines.

### v0.2.0 - Target Matrix Skeleton

Goal:

Make the supported 1.0 target matrix explicit before backend work starts.

Deliverables:

- Target identifiers for `x86`, `x86_64`, `aarch32`, and `aarch64`.
- ELF32 and ELF64 format identifiers.
- Windows x86_64 PE64 target identifier.
- macOS Apple Silicon aarch64 Mach-O target identifier.
- CLI-visible target names documented.
- Rejection diagnostics for unsupported OS/format pairs.

Verification:

- Unit tests prove all 1.0 targets are represented.
- Unsupported target strings fail with stable diagnostics.

### v0.3.0 - Source And Diagnostics Core

Goal:

Build the source and diagnostic substrate used by every compiler phase.

Deliverables:

- Source byte model.
- Span model.
- Line/column lookup.
- Diagnostic codes.
- Diagnostic severity.
- Diagnostic rendering contract.
- Compile limits.

Verification:

- Span lookup tests.
- Diagnostic golden tests.
- Program-size limit tests.

## Phase 1: Compiler Substrate

### v0.4.0 - Compiler Pipeline Skeleton

Goal:

Build the non-language-specific pipeline shape before adding real BASIC
semantics.

Deliverables:

- Pipeline stage traits or function boundaries.
- Source-to-diagnostic contract.
- HIR-to-MIR boundary.
- MIR-to-LIR boundary.
- LIR-to-target boundary.
- Report sink contract.

Verification:

- Empty pipeline tests.
- Stage ordering tests.
- Error propagation tests.

### v0.5.0 - Source Normalization Core

Goal:

Normalize raw source bytes into line-oriented text without parsing BASIC.

Deliverables:

- Line ending normalization.
- ASCII/control-character policy.
- Blank-line policy.
- Source-size limit enforcement.
- Stable source ID model.

Verification:

- CRLF/LF/CR fixtures.
- Invalid byte fixtures.
- Large source rejection tests.

### v0.6.0 - Diagnostics And Reporting Core

Goal:

Make diagnostics and reports stable before parsing starts.

Deliverables:

- Diagnostic code registry.
- Severity model.
- Source snippet rendering contract.
- Report section model.
- Golden-test harness for diagnostics and reports.

Verification:

- Diagnostic golden tests.
- Report golden tests.
- No panic on malformed source snippets.

### v0.7.0 - HIR/MIR/LIR Core Contracts

Goal:

Define compiler representations without committing to later Dartmouth versions.

Deliverables:

- HIR ID model.
- MIR value and label IDs.
- LIR label and symbol IDs.
- Validation contracts for each layer.
- Version-neutral lowering interfaces.

Verification:

- Representation construction tests.
- Validator failure tests.

### v0.8.0 - Runtime Fragment Model

Goal:

Represent generated-program helpers without linking an external runtime.

Deliverables:

- Runtime fragment inventory.
- Fragment dependency graph.
- Fragment symbol naming.
- Fragment inclusion report.

Verification:

- Fragment selection tests.
- No-unused-fragment tests.
- Fragment inclusion report tests.
- Non-emission contract tests.

### v0.9.0 - Manual Corpus Setup

Goal:

Turn the local Dartmouth manuals into controlled fixture sources without
committing restricted documents.

Deliverables:

- Fixture directory layout.
- `docs/languages/` language-reference layout.
- Dartmouth BASIC 1 reference document written in Elderheim's own words.
- `examples/dartmouth-basic-1/` committed example corpus.
- Manual provenance notes for versions 1, 2, and 4.
- No committed proprietary/manual PDF content unless license allows it.
- Extracted tiny examples where allowed or handwritten equivalent fixtures.
- Documentation that version 3 is out of scope.

Verification:

- Fixture manifest validates.
- Dartmouth BASIC 1 examples are included in tests.
- Missing manual paths produce clear local-only warnings, not CI failures.

## Phase 2: Dartmouth BASIC 1 Complete Frontend

### v0.10.0-elderheim - BASIC 1 Line Table

Goal:

Represent numbered Dartmouth BASIC 1 source safely.

Deliverables:

- Line number parser.
- Line table.
- Duplicate line-number diagnostics.
- Out-of-order line-number policy for BASIC 1.
- Empty numbered line policy.

Verification:

- BASIC 1 line table tests.
- Duplicate and malformed line-number tests.

### v0.11.0 - BASIC 1 Lexer

Goal:

Lex BASIC 1 fixtures according to the available first-edition manual.

Deliverables:

- BASIC 1 token kinds.
- BASIC 1 keyword set.
- Token span storage.
- Identifier policy.
- Unknown character diagnostics.

Verification:

- BASIC 1 token exact-output golden tests.
- Manual-derived lexer fixtures.

### v0.12.0 - BASIC 1 HIR Shape

Goal:

Define the source-shaped BASIC 1 representation. This release used the name
HIR; `v0.13.2` renames the token-bearing form to CST before downstream semantic
HIR depends on that terminology.

Deliverables:

- Program HIR.
- Line HIR.
- Statement HIR for BASIC 1 only.
- Expression HIR for BASIC 1 only.
- HIR debug/report snapshots.

Verification:

- HIR construction tests.
- BASIC 1 HIR snapshot tests.

### v0.13.0 - BASIC 1 Minimal Parser

Goal:

Parse the smallest executable BASIC 1 program shape.

Deliverables:

- `PRINT` parser for supported BASIC 1 forms.
- `END` parser.
- Statement terminator handling.
- Parse diagnostics.

Verification:

- `10 PRINT "HELLO"` fixture if strings are supported by the selected BASIC 1
  subset; otherwise the earliest manual-supported PRINT fixture.
- `20 END` fixture.
- Malformed PRINT tests.

### v0.13.1 - Normalized Frontend And Absolute Spans

Goal:

Close every public path that can bypass source normalization and make all
frontend locations directly renderable against the normalized source.

Deliverables:

- A dialect-bound `NormalizedSource<D>` capability produced only by checked
  decoding, normalization, byte-policy, and compile-limit validation.
- Public line-table, lexer, CST, parser, and semantic entry points accept the
  capability instead of arbitrary `&str` input.
- Raw-text construction helpers are private or test-only and cannot reach
  production compilation.
- Every line-table entry carries its absolute normalized-source range.
- Lexer and nested frontend errors translate statement-local positions to
  absolute source spans without discarding line or secondary context.
- The committed BASIC 1 example manifest and compiled fixture inventory are
  checked for exact two-way parity, including duplicate and unlisted files.

Verification:

- All 256 byte values, malformed CR/LF sequences, NUL/control bytes, Unicode
  formatting controls, size limits, and line limits are accepted or rejected
  deterministically by the normalization boundary.
- Direct frontend callers cannot construct an unchecked source capability.
- Golden diagnostics point to exact absolute spans across multiple lines and
  preserve nested lexer/parser context.
- Removing, duplicating, or adding an unlisted corpus entry fails the manifest
  parity test.
- No CST, AST, semantic HIR, or later-stage value is produced after source
  rejection.

### v0.13.2 - CST Ownership And Complete Pipeline Capabilities

Goal:

Make compiler-stage omission structurally impossible and give each source
representation one unambiguous owner and meaning.

Deliverables:

- Rename the Dartmouth token-bearing source representation from HIR to CST or
  token tree, including APIs, diagnostics, snapshots, tests, and documentation.
- Keep Dartmouth CST, AST, and semantic HIR frontend-owned and dialect-aware.
- Remove shared generic HIR from `elderheim-ir` unless a concrete second
  frontend proves a language-neutral requirement; shared IR begins at MIR.
- Replace freely supplied stage slices with a fixed capability chain:
  normalized source, parsed dialect AST, validated semantic HIR, validated MIR,
  validated target LIR, encoded program, validated image plan, verified image.
- Each lowerer, backend, relocation resolver, and writer accepts only the
  validated output type of its immediate predecessor.
- Error states consume or invalidate builders so compilation cannot resume at
  a later stage.
- Crate-local `#![forbid(unsafe_code)]` is permanent in production compiler,
  frontend, IR, runtime, backend, and writer crates; policy gates reject build
  scripts, FFI, native linking, inline assembly, and production process
  spawning.

Verification:

- Compile-fail API tests prove stages cannot be skipped, reordered, forged, or
  mixed across dialects or targets.
- Empty and single-stage pipelines no longer exist as successful public paths.
- Failure injection at every stage proves no later-stage event or artifact is
  emitted.
- CST snapshots remain stable after the terminology migration and are clearly
  separate from semantic HIR reports.

### v0.13.3 - Unified Budgets And Transactional Builders

Goal:

Apply one compilation-wide resource policy and prevent failed construction from
leaking partial compiler state.

Deliverables:

- One `CompileLimits` flow for normalized bytes, lines, tokens, CST/AST/HIR
  nodes, MIR/LIR operations, diagnostics, recursion or nesting, emitted code,
  relocation count, and output image bytes.
- Reconcile the current 262,144-operation core default with the 65,536 MIR/LIR
  caps; every effective limit comes from the validated compilation policy.
- Bounded builders own partial CST, AST, semantic HIR, MIR, and LIR output and
  expose only a consuming `finish()` that validates before publication.
- Compiler-controlled vectors reserve against checked budgets with
  `try_reserve_exact` where allocation failure can be reported.
- Dense contiguous IDs use bounded bitsets or indexed tables; adversarial raw
  IDs use bounded collect/sort/deduplicate/search validation.
- A bounded diagnostic budget may collect related frontend errors, but any
  error prevents semantic HIR and code generation.
- A complexity ledger records worst-case time, memory, worklist, recursion, and
  iteration bounds for expression parsing, symbol/line resolution, CFG and
  dominance construction, runtime dependency closure, register allocation,
  branch relaxation, interval layout, and independent verification.
- No unbounded fixpoint. Quadratic behavior over user-controlled structures is
  rejected unless a measured worst-case under the enforced cap is explicitly
  approved in the ledger.

Verification:

- Boundary tests cover every budget at limit minus one, limit, and limit plus
  one, including checked multiplication and `usize` conversion.
- Allocation and sink failure injection never exposes partial output.
- Validation complexity and memory baselines are recorded for maximum-size
  accepted inputs.
- Diagnostics are deterministic across repeated runs and stop at the configured
  budget.
- Complexity regression fixtures exercise each algorithm at representative and
  maximum accepted sizes.

### v0.13.4 - Sealed Dartmouth Edition Profiles

Goal:

Freeze manual-backed dialect decisions before expression and control-flow
parsing grows around mutable profile fields.

Deliverables:

- A closed `DartmouthEdition`/profile API for versions 1, 2, and 4; callers
  cannot assemble unsupported mixtures of rules.
- Central manual-derived tables for line-number rules, keywords, statement
  availability, identifiers, arrays/default bounds, PRINT, DATA/READ, and
  control-flow behavior.
- Edition-specific initial-state rules for scalars, arrays, DATA cursors, loop
  variables, subroutine state, and deterministic randomness.
- Explicit introduced, removed, and behavior-changed metadata per edition.
- BASIC 3 remains a named but unconstructable unsupported edition with a stable
  primary-source diagnostic.
- Shared grammar machinery uses explicit profile decisions; rules that truly
  differ remain separate and testable implementations.
- Every rule links to an Elderheim-authored manual identifier and source/page
  reference without copying manual prose.

Verification:

- Public construction tests prove only supported sealed profiles exist.
- Table completeness and duplicate/conflict checks pass for all mapped rules.
- Cross-version fixtures prove earlier editions reject every mapped later-only
  feature.
- Profile snapshots and manual-rule links are deterministic.

### v0.13.5 - BASIC 1 Historical Numeric Model

Goal:

Specify BASIC 1 numeric behavior before numeric parsing, semantic validation,
MIR, and runtime implementation depend on host-language assumptions.

Deliverables:

- Manual-backed BASIC 1 rules for literal syntax, representation, precision,
  rounding, exponent limits, overflow, underflow, comparison, and integer
  conversion.
- A deterministic safe-Rust reference numeric model independent of host
  floating-point behavior where the manual contract requires it.
- Stable numeric errors and manual-rule identifiers.
- A documented delta mechanism for BASIC 2 and BASIC 4 numeric changes; those
  editions cannot silently inherit BASIC 1 behavior.
- Serialization and report forms that are deterministic across host targets.

Verification:

- Manual-derived numeric vectors cover zero, signs, precision boundaries,
  exponent endpoints, rounding ties, overflow, underflow, and comparison.
- Repeated and cross-host runs produce identical reference results and reports.
- Host floating-point shortcuts that disagree with the selected historical
  model are rejected by tests.
- Mutation around every numeric boundary yields a result or structured error,
  never a panic or partial semantic value.

### v0.13.6 - Manual Provenance And Coverage Ledger

Goal:

Make every language claim traceable in both directions without redistributing
copyrighted manual text.

Deliverables:

- Stable provenance metadata for each manual: normalized title, edition,
  publication date, scan or text-export form, page-numbering convention, and a
  content fingerprint when legally permissible.
- A documented fingerprint algorithm and normalization policy that does not
  depend on local filenames or filesystem metadata.
- An errata and ambiguity register with stable IDs, evidence, selected
  interpretation, affected editions, and compatibility consequences.
- A two-way rule ledger: every implemented/rejected language rule maps to a
  manual rule, and every in-scope manual rule maps to implemented behavior, an
  explicit rejection, or a source-language scope exclusion.
- Corpus fixtures link edition, provenance ID, rule ID, page/reference, expected
  result, and expected rejection mode.
- Coverage reports reject unknown, duplicate, orphaned, or contradictory rule
  and provenance IDs.

Verification:

- Provenance snapshots are stable across machines and local manual filenames.
- Changing manual content changes its permitted fingerprint while changing
  file metadata does not.
- Removing either side of a rule mapping fails the two-way ledger gate.
- Errata/ambiguity decisions have complete references and deterministic report
  ordering.
- Documentation contains Elderheim-authored summaries rather than copied
  manual prose.

### v0.13.7 - Versioned Direct Target-Service Feasibility

Goal:

Prove every promised 1.0 target has a documented stable no-import service path
before further language and backend investment assumes that target is feasible.

Deliverables:

- A canonical target-service contract representation containing architecture,
  execution mode, OS family and supported version range, service ABI revision,
  and independent revisions for process entry, output, input, failure, and
  termination.
- `TargetServiceContractId` combines the human-reviewed logical revision with a
  domain-separated content fingerprint of that canonical representation. The
  fingerprint is recomputed during validation rather than trusted as metadata.
- The canonical field order, integer/string encoding, domain tag, digest
  algorithm, and output width are fixed and versioned; ambiguous encodings and
  alternate algorithms are rejected.
- Each contract records register inputs/results/clobbers, stack state, accessible
  memory, pointer/length rules, error results, retry/progress rules, and exit
  behavior.
- Primary authoritative evidence that each direct service transition is
  documented and stable for Linux x86/x86_64/AArch32/AArch64, Windows x86_64,
  and macOS AArch64 under the promised version range.
- No dynamic/static OS-library import, external binary, library, hook, linker,
  compiler, or runtime mechanism is admitted by a service contract.
- In-process service-state models and checked-in minimal entry/write/input/
  failure/exit transition vectors for every target. They establish feasibility
  without claiming a complete encoder or native executable.
- Closed `SupportedTarget` and `TargetSpec` types with private fields and
  validated constructors. Each supported target binds exactly one canonical
  service-contract ID and fingerprint; unknown, ambiguous, superseded,
  forged, or cross-target combinations cannot construct a supported value.
- If any promised target lacks a stable direct contract, this stop is blocked
  and requires an explicit project-scope resolution release before `v0.14.0`;
  later backend work cannot reinterpret the no-import rule.

Verification:

- Canonical contract bytes, logical IDs, and fingerprints round-trip
  deterministically and are independent of map order, host, and filesystem
  metadata.
- Independent fingerprint recomputation rejects a stale supplied fingerprint
  even when the logical revision was not updated, and proves every observable
  ABI/service rule change changes the effective contract identity.
- Target, mode, OS range, revision, register, stack, memory, and error-rule
  mutation tests fail validation independently.
- In-process vectors cover entry, successful/partial/failed output and input,
  EOF, runtime failure, and termination for every promised target.
- Cross-revision and cross-target service plans fail even when their broad
  architecture/OS names match.
- Feasibility reports name all evidence, assumptions, unresolved platform
  risks, and a PASS/BLOCKED decision per target without invoking external
  executables.
- Checked-in independently derived known-answer vectors cover canonical
  target-service encodings and fingerprints for empty/minimum/maximum legal
  fields plus every supported target contract.
- A separate strict decoder rejects malformed lengths, alternate field order,
  duplicate fields, trailing bytes, unknown fields, and noncanonical version
  or integer encodings. It does not call the production canonical encoder or
  fingerprint helper when recomputing vector results.

### v0.14.0 - BASIC 1 LET, Variables, And Numbers

Goal:

Parse BASIC 1 assignment and numeric expressions.

Deliverables:

- BASIC 1 variable identifier rules.
- `LET` parser.
- Numeric literal parser.
- Expression parser.
- Overflow diagnostics.

Verification:

- BASIC 1 assignment fixtures.
- BASIC 1 expression precedence fixtures.
- Overflow rejection tests.
- Long digit runs, exponent edges, quote/token storms, malformed operators,
  nesting limits, and token-budget boundaries fail deterministically.

### v0.15.0 - BASIC 1 Control Flow Parser

Goal:

Parse BASIC 1 control flow exactly as supported by the first-edition manual.

Deliverables:

- BASIC 1 `GOTO` policy.
- BASIC 1 `IF THEN` policy.
- BASIC 1 loop/subroutine policy if present in the supported first-edition
  subset.
- Stable unsupported-feature diagnostics for later-version constructs.

Verification:

- BASIC 1 control-flow fixtures.
- Later-version syntax rejection fixtures.
- Missing operands, extreme line targets, malformed nesting, and diagnostic
  budget tests produce no semantic HIR.

### v0.16.0 - BASIC 1 Semantic Validation

Goal:

Validate complete BASIC 1 programs before MIR lowering.

Deliverables:

- BASIC 1 feature matrix.
- Variable resolver.
- Line label resolver.
- Control-flow graph.
- Reachability report.
- Edition-aware initialization lowering: any manual-defined implicit initial
  value becomes an explicit semantic-HIR/MIR entry definition.
- Arrays, DATA cursors, loop variables, subroutine state, and RND state follow
  the sealed edition profile rather than modern-language assumptions.
- Stable language/stage diagnostic codes with manual-rule identifiers.
- Bounded multi-diagnostic analysis that invalidates semantic output if any
  error is present.

Verification:

- Missing target diagnostics.
- Unreachable line report tests.
- BASIC 1 feature matrix tests.
- Historically valid implicit-initialization fixtures lower to explicit entry
  definitions, while genuinely undefined uses retain stable diagnostics.
- Primary/secondary absolute-span golden tests and deterministic diagnostic
  ordering at the configured budget.

### v0.16.1 - Typed Block-Structured MIR

Goal:

Replace the linear integer-only MIR scaffold with the complete validated
target-neutral structure needed by historical BASIC semantics.

Deliverables:

- Explicit basic blocks with typed block parameters and one mandatory
  terminator per block.
- Typed values and operations for the selected historical numeric model,
  control flow, data references, calls, memory regions, and runtime capability
  requests.
- Declared `DataId` objects, function signatures, call arguments/results, and
  reachable exit behavior.
- Deterministic validation phases: shape, definitions, references/types,
  CFG/reachability, dominance/definite assignment, data/calls, runtime
  capabilities, and exits.
- No Unix syscall, concrete register, relocation, ABI, or executable-format
  concepts in MIR.
- MIR definite assignment remains strict and edition-neutral because the
  frontend explicitly materializes every profile-defined initial state.

Verification:

- Tests independently reject use-before-definition, non-dominating values,
  type mismatch, undefined data, bad call signatures, unterminated blocks,
  malformed successors, unreachable invalid blocks, missing capabilities, and
  absent reachable exits.
- Scalar, array, DATA, loop, subroutine, and RND initialization fixtures prove
  edition-valid source is not rejected by MIR's machine-level rules.
- Maximum-size valid CFGs remain within recorded time and memory bounds.
- Raw MIR mutation reaches every validator error code without panic.
- A small-state model agrees with definition/reference, termination,
  reachability, and dominance validation.

### v0.16.2 - Transactional MIR Construction And Oracle

Goal:

Publish MIR only after complete bounded construction and independent semantic
agreement.

Deliverables:

- A bounded MIR builder issuing dense IDs and owning all partial blocks, data,
  values, signatures, and capabilities.
- Fallible capacity reservation before compiler-controlled growth.
- Consuming `finish() -> Result<ValidatedMir, _>`; raw partial MIR is not a
  backend input.
- Atomic lowering operations that stage changes before committing them to the
  builder.
- A deliberately independent MIR interpreter that reports observable output,
  input consumption, runtime failure, and exit status.

Verification:

- Failure injection at every builder operation leaves no publishable MIR.
- Dense-ID, overflow, capacity, and operation-budget boundary tests pass.
- Valid MIR traces agree with the Dartmouth semantic reference model for the
  currently implemented BASIC 1 corpus.
- Mutated MIR either validates and executes deterministically or returns a
  structured validation error before interpretation.

### v0.16.3 - Structured Frontend Diagnostics

Goal:

Freeze diagnostics as a language and security contract before MIR lowering.

Deliverables:

- Stable Dartmouth/stage-specific diagnostic codes rather than only broad core
  categories.
- Absolute primary spans, bounded secondary spans, edition requirements, and
  manual-rule identifiers.
- Deterministic diagnostic ordering, deduplication, and truncation reporting.
- Reports distinguish source, CST, AST, semantic, MIR, target, and image
  failures without embedding unescaped source controls.
- Any error result structurally withholds validated semantic HIR.

Verification:

- Golden tests cover every diagnostic code and rendering mode.
- Multi-error fixtures prove deterministic bounded collection and no codegen.
- Span, source-identity, edition, and manual-rule mismatches fail closed.
- Escape, bidi, control-byte, and escape-looking text tests preserve canonical
  unambiguous reports.

### v0.16.4 - Validated MIR Transformation Policy

Goal:

Prevent optimization or canonicalization from becoming an unvalidated path
around MIR invariants, even when 1.0 intentionally uses few transformations.

Deliverables:

- A sealed transformation interface that consumes `ValidatedMir` and builds a
  separate candidate through the bounded transactional MIR builder.
- Every pass revalidates shape, types, CFG, dominance, data, calls,
  capabilities, and exits before returning a new `ValidatedMir`.
- Validated artifacts are immutable; no pass mutates one in place or exposes
  unchecked internal collections.
- Constant folding and algebraic simplification use only the historical numeric
  model and its exact failure/rounding behavior.
- Dead-code handling preserves required unreachable-line diagnostics,
  compatibility reports, DATA semantics, and observable failure behavior.
- Pass count, per-pass growth, worklist size, and iteration count are bounded by
  `CompileLimits` and reported.

Verification:

- Before/after observable traces agree in the independent MIR interpreter for
  every transformation fixture.
- Failure injection and invalid transformed output publish no replacement MIR.
- Historical-number boundary vectors prevent host-arithmetic folding.
- Dead-code fixtures preserve required reports and edition behavior.
- Pass-order and repeated-run tests are deterministic and terminate within the
  documented complexity bounds.

### v0.17.0 - BASIC 1 MIR Lowering

Goal:

Lower BASIC 1 into target-neutral MIR.

Deliverables:

- `WriteStatic` or numeric `Write` lowering as appropriate.
- Variable load/store lowering.
- Arithmetic lowering.
- Branch/jump lowering.
- MIR validator coverage for BASIC 1.

Verification:

- BASIC 1 MIR golden tests.
- MIR validator failure tests.

### v0.18.0 - BASIC 1 Runtime Requirements

Goal:

Define exactly which runtime fragments BASIC 1 needs.

Deliverables:

- Write/exit runtime contracts.
- Historical-number parsing and formatting requirements.
- Numeric function and deterministic randomness requirements.
- Runtime error policy.
- Fragment inventory report.

Verification:

- BASIC 1 runtime inventory tests.
- Runtime report golden tests.

### v0.18.1 - Portable Observable Runtime Contract

Goal:

Freeze user-observable behavior before runtime fragments and platform backends
can make incompatible assumptions.

Deliverables:

- Exact output-byte semantics and newline policy.
- Input character set, line delimiters, maximum line/token sizes, and
  normalization rules.
- EOF, partial input, malformed input, and service-error behavior.
- Stable runtime-error messages and error-to-exit-status mapping.
- A bounded partial read/write policy with progress requirements, retry limits,
  and no unbounded loops.
- A target-deviation schema; any permitted platform difference is explicit in
  compatibility and generated-binary reports.

Verification:

- Portable output/input traces are exact-byte golden fixtures.
- EOF and every partial read/write/service-error branch terminates
  deterministically within its retry bound.
- Newline, encoding, delimiter, error, and exit semantics are identical across
  target models unless a declared deviation applies.
- No target backend can override the portable contract without a validated
  deviation entry.

### v0.18.2 - Historical Numeric Operations And Functions

Goal:

Implement the complete BASIC 1 numeric operation/function semantics in the
safe-Rust reference model before runtime or backend code duplicates them.

Deliverables:

- A model-neutral `HistoricalNumber` semantic/MIR contract; no public
  `i64`-specific shortcut defines language behavior.
- Deterministic implementations of `ABS`, `ATN`, `COS`, `EXP`, `INT`, `LOG`,
  `SIN`, `SQR`, and `TAN` where required by the First Edition ledger.
- Domain, range, overflow, underflow, rounding, precision, and conversion
  failures tied to stable manual-rule IDs.
- A recorded applicability decision for signed zero, exceptional values, and
  underflow behavior under the selected historical model.
- Deterministic approximation algorithms and constants implemented in safe
  Rust without host math libraries.

Verification:

- Manual-derived and independently calculated vectors cover normal values,
  boundaries, identities, quadrants, approximation transitions, and every
  failure domain.
- Results and failures are byte-identical across supported compiler hosts.
- Host `libm`/platform floating-point behavior is not a production dependency.
- Function mutation and iteration-limit tests terminate without panic.

### v0.18.3 - Number I/O And Deterministic Randomness

Goal:

Complete edition-specific number parsing, formatting, and BASIC 1 randomness
before the independent semantic oracle closes the profile.

Deliverables:

- `parse_number` and `print_number` contracts using `HistoricalNumber` and the
  portable observable runtime policy.
- Edition-specific signs, spacing, exponent notation, significant digits,
  rounding, delimiters, overflow text, and invalid-input behavior.
- Fixed maximum numeric-token and formatting-buffer sizes with checked
  pointer-plus-length ranges.
- `RND` seed, reseed, sequence, repeatability, range, and error behavior where
  required by the First Edition rule ledger.
- A deterministic PRNG/reference sequence implemented in safe Rust without
  operating-system randomness or host-library state.

Verification:

- Parse/format round trips and exact output vectors cover every numeric class
  and boundary permitted by the historical model.
- Oversized tokens, buffers, partial input, malformed exponents, and retry
  limits fail with stable runtime results.
- Fixed seeds produce identical sequences across hosts and targets.
- The rule ledger explicitly records `RND` behavior or its absence for every
  supported Dartmouth edition.

### v0.18.4 - Semantic Contract And Compilation Identity

Goal:

Give every validated compiler artifact an opaque, content-bound identity for
the exact source-language semantics and compilation policy that produced it
before the independent oracle becomes a release gate.

Deliverables:

- A `SemanticContractId` containing a logical schema revision plus a
  domain-separated fingerprint of a deterministic canonical semantic contract.
- The semantic contract binds the Dartmouth edition; complete edition rule
  tables; selected errata/ambiguity decisions; historical-number model,
  representation, algorithms, constants, and failure rules; parsing/formatting
  rules; deterministic RND contract; and portable observable-runtime revision.
- A separate opaque `CompilationIdentity` binds `SemanticContractId`, normalized
  source digest, frontend/MIR/LIR schema versions, semantics-affecting compiler
  options and transformation set, and the complete effective `CompileLimits`
  configuration.
- The normalized source digest is a domain-separated cryptographic digest of
  normalized source bytes under a fixed algorithm; it is distinct from the
  diagnostic-only `SourceId` and is never reconstructed from that identifier.
- Fixed and versioned canonical field order, integer/string/list encoding,
  domain tags, digest algorithms, and output widths for both identities.
  Unknown fields, ambiguous encodings, alternate algorithms, and partial
  identities are rejected.
- Frontend-owned validated semantic HIR constructs the identity. Semantic HIR,
  MIR, `RuntimePlan`, final LIR, `ResourcePlan`, `ResourceCertificate`,
  `VerifiedImage`, oracle fixtures, compatibility evidence, and reports carry
  the same opaque identity without reconstruction from partial fields.
- Shared IR, optimization, runtime, and backend crates may compare, copy, and
  report the opaque identity but cannot inspect Dartmouth rule fields to choose
  behavior. Dialect decisions remain in the language frontend.
- Builders and transformations reject inputs with different identities, and
  any failure or mutation invalidates the candidate artifact rather than
  retaining the predecessor identity.
- BASIC 1 receives a complete identity at this stop. BASIC 2 and BASIC 4 must
  receive distinct complete identities as their edition deltas close at
  `v0.27.0` and `v0.34.0`; an incomplete profile cannot claim one.

Verification:

- Independently derived checked-in known-answer vectors cover canonical
  semantic contracts and compilation identities, including minimum and
  maximum effective limits and representative normalized sources.
- A strict independent decoder/fingerprinter does not reuse production
  canonicalization helpers and rejects empty/truncated fields, malformed
  lengths, alternate field order, duplicate/trailing/unknown fields, and
  noncanonical version or integer encodings.
- Mutating each rule table, errata decision, numeric constant/algorithm, RND
  rule, runtime revision, schema version, option, source byte, or effective
  limit changes the appropriate identity; stale supplied fingerprints fail.
- LF, CRLF, and CR inputs that normalize to identical source bytes produce the
  same source digest, while any normalized-byte difference changes it.
- Compile-fail/API-boundary tests prove shared mid-end and backend code cannot
  inspect dialect-specific identity internals or forge an identity.
- Cross-identity HIR/MIR/runtime/LIR/resource/image composition fails at every
  boundary with no partial artifact or report presented as successful.
- Repeated and cross-host computations produce byte-identical IDs and reports.

### v0.19.0 - Independent BASIC 1 Semantic Oracle

Goal:

Prove complete BASIC 1 semantics with an implementation independent from
production lowering, runtime fragments, and native backends.

Deliverables:

- A pure safe-Rust Dartmouth BASIC 1 semantic interpreter.
- Independent control flow, variables, historical numbers, functions, arrays,
  DATA state, input/output, errors, and exit behavior.
- Observable traces for output bytes, input consumption, runtime failures, and
  exit status.
- No reuse of production MIR lowering, runtime fragment, encoder, or writer
  implementation logic.

Verification:

- Every BASIC 1 semantic fixture and manual rule passes in the independent
  oracle.
- Production semantic HIR and MIR traces agree with the oracle.
- Deliberately injected production semantic defects are detected by oracle
  comparison tests.
- No native executable output is claimed by this tag.

### v0.20.0-elderheim - BASIC 1 Compatibility Sweep

Goal:

Close known BASIC 1 frontend and semantic gaps before adding BASIC 2.

Deliverables:

- BASIC 1 supported-feature matrix.
- BASIC 1 completion-blocker register; every item is either fixed before this
  tag or assigned to an explicit follow-up release.
- BASIC 1 manual-derived fixture suite.
- BASIC 2 and BASIC 4 syntax rejection suite.

Verification:

- BASIC 1 fixture suite passes.
- Cross-version rejection suite passes.

## Phase 3: Dartmouth BASIC 2 Complete Expansion

### v0.21.0 - BASIC 2 Manual Delta

Goal:

Identify exactly what BASIC 2 changes relative to BASIC 1.

Deliverables:

- BASIC 2 delta document.
- BASIC 2 feature matrix.
- BASIC 1 compatibility impact review.
- BASIC 2 fixture manifest.

Verification:

- Delta document links to fixtures.
- BASIC 1 tests still pass.

### v0.22.0 - BASIC 2 Lexer Expansion

Goal:

Add only the BASIC 2 lexical additions.

Deliverables:

- BASIC 2 keyword additions.
- BASIC 2 token policy changes.
- BASIC 1 mode rejection for BASIC 2-only tokens.

Verification:

- BASIC 2 lexer fixtures.
- BASIC 1 cross-version rejection fixtures.

### v0.23.0 - BASIC 2 Parser Expansion

Goal:

Add BASIC 2 parser support without changing BASIC 1 behavior.

Deliverables:

- BASIC 2 statement additions.
- BASIC 2 expression additions.
- Version-gated parser rules.
- Stable diagnostics for unsupported version combinations.

Verification:

- BASIC 2 parser fixtures.
- BASIC 1 parser regression suite.

### v0.24.0 - BASIC 2 Semantic Validation

Goal:

Validate complete BASIC 2 programs.

Deliverables:

- BASIC 2 resolver additions.
- BASIC 2 control-flow additions.
- BASIC 2 numeric semantics changes.
- BASIC 2 report sections.

Verification:

- BASIC 2 semantic fixtures.
- BASIC 1 semantic regression suite.

### v0.25.0 - BASIC 2 MIR And Runtime Delta

Goal:

Lower BASIC 2 additions into MIR and runtime fragments.

Deliverables:

- BASIC 2 MIR lowering additions.
- Runtime fragment additions.
- Runtime inventory deltas.
- Validator coverage.

Verification:

- BASIC 2 MIR golden tests.
- BASIC 1 MIR regression suite.

### v0.26.0 - Independent BASIC 2 Semantic Oracle

Goal:

Extend the independent semantic oracle through every BASIC 2 delta before
moving to BASIC 4.

Deliverables:

- Independently implemented BASIC 2 semantic behavior.
- BASIC 2 stdin/stdout fixtures.
- BASIC 2 runtime-error fixtures.
- BASIC 2 historical-number, function, formatting, and randomness deltas.
- Independently recomputed BASIC 2 `SemanticContractId` known-answer vectors.

Verification:

- BASIC 2 semantic fixtures agree across the oracle, semantic HIR, and MIR.
- BASIC 1 oracle and production suites still pass unchanged.

### v0.27.0 - BASIC 2 Compatibility Sweep

Goal:

Close known BASIC 2 gaps before adding BASIC 4.

Deliverables:

- BASIC 2 supported-feature matrix.
- BASIC 2 completion-blocker register; every item is either fixed before this
  tag or assigned to an explicit follow-up release.
- BASIC 2 manual-derived fixture suite.
- BASIC 4 syntax rejection suite in BASIC 2 mode.
- Final content-bound BASIC 2 `SemanticContractId` and representative
  `CompilationIdentity` fixtures; neither may equal the BASIC 1 identity.

Verification:

- BASIC 1 fixture suite passes.
- BASIC 2 fixture suite passes.
- Cross-version rejection suite passes.
- Every BASIC 2 fixture/report carries the final BASIC 2 identity, while BASIC
  1 fixtures retain their unchanged identity.

## Phase 4: Dartmouth BASIC 4 Complete Expansion

### v0.28.0 - BASIC 4 Manual Delta

Goal:

Identify exactly what BASIC 4 changes relative to BASIC 2.

Deliverables:

- BASIC 4 delta document.
- BASIC 4 feature matrix.
- BASIC 1 and BASIC 2 compatibility impact review.
- BASIC 4 fixture manifest.

Verification:

- Delta document links to fixtures.
- BASIC 1 and BASIC 2 tests still pass.

### v0.29.0 - BASIC 4 Lexer Expansion

Goal:

Add only the BASIC 4 lexical additions.

Deliverables:

- BASIC 4 keyword additions.
- BASIC 4 token policy changes.
- BASIC 1/BASIC 2 mode rejection for BASIC 4-only tokens.

Verification:

- BASIC 4 lexer fixtures.
- BASIC 1 and BASIC 2 lexer regression suites.

### v0.30.0-elderheim - BASIC 4 Parser Expansion

Goal:

Add BASIC 4 parser support without changing older-version behavior.

Deliverables:

- BASIC 4 statement additions.
- BASIC 4 expression additions.
- Version-gated parser rules.
- Stable diagnostics for unsupported version combinations.

Verification:

- BASIC 4 parser fixtures.
- BASIC 1 and BASIC 2 parser regression suites.

### v0.31.0 - BASIC 4 Semantic Validation

Goal:

Validate complete BASIC 4 programs.

Deliverables:

- BASIC 4 resolver additions.
- BASIC 4 control-flow additions.
- BASIC 4 numeric semantics changes.
- BASIC 4 report sections.

Verification:

- BASIC 4 semantic fixtures.
- BASIC 1 and BASIC 2 semantic regression suites.

### v0.32.0 - BASIC 4 MIR And Runtime Delta

Goal:

Lower BASIC 4 additions into MIR and runtime fragments.

Deliverables:

- BASIC 4 MIR lowering additions.
- Runtime fragment additions.
- Runtime inventory deltas.
- Validator coverage.

Verification:

- BASIC 4 MIR golden tests.
- BASIC 1 and BASIC 2 MIR regression suites.

### v0.33.0 - Independent BASIC 4 Semantic Oracle

Goal:

Extend the independent semantic oracle through every BASIC 4 delta before
native backend work becomes the main focus.

Deliverables:

- Independently implemented BASIC 4 semantic behavior.
- BASIC 4 stdin/stdout fixtures.
- BASIC 4 runtime-error fixtures.
- BASIC 4 historical-number, function, formatting, and randomness deltas.
- Independently recomputed BASIC 4 `SemanticContractId` known-answer vectors.

Verification:

- BASIC 1, BASIC 2, and BASIC 4 semantic fixtures agree across the independent
  oracle, semantic HIR, and MIR.

### v0.34.0 - BASIC 4 Compatibility Sweep

Goal:

Close known BASIC 4 gaps before platform backend work.

Deliverables:

- BASIC 4 supported-feature matrix.
- BASIC 4 completion-blocker register; every item is either fixed before this
  tag or assigned to an explicit follow-up release.
- BASIC 4 manual-derived fixture suite.
- Cross-version matrix for BASIC 1, BASIC 2, and BASIC 4.
- Final content-bound BASIC 4 `SemanticContractId` and representative
  `CompilationIdentity` fixtures distinct from BASIC 1 and BASIC 2.

Verification:

- BASIC 1 fixture suite passes.
- BASIC 2 fixture suite passes.
- BASIC 4 fixture suite passes.
- Cross-version rejection suite passes.
- Every fixture/report carries the exact edition identity and cross-edition
  artifact composition fails closed.

### v0.34.1 - Closed Target Capability And ABI Integration

Goal:

Integrate the already-closed target and service-contract types from `v0.13.7`
with runtime capabilities and freeze the ABI vocabulary before target-near
lowering begins.

Deliverables:

- Runtime, LIR, backend, and report APIs accept only the closed
  `SupportedTarget` and `TargetSpec` values proven at `v0.13.7`; no second or
  weaker target-construction path is introduced.
- The existing closed target identity binds architecture, mode, operating
  system, executable format/class, endianness, ABI, service convention,
  pointer width, logical service revision, and canonical contract fingerprint.
- `TargetSpec`, target capabilities, `RuntimePlan<Target>`, and compatibility
  reports carry the exact `TargetServiceContractId` proven at `v0.13.7`.
- Typed target capabilities declare available write, read, terminate, memory,
  stack, and failure services.
- A complete per-target process-entry, output, input, termination, and failure
  path that satisfies the portable observable runtime contract.
- External compiler, runtime, tool, C-library, and BASIC-runtime dependencies
  are forbidden.
- Dynamic and static OS-library imports are forbidden for supported generated
  programs; PE import tables and Mach-O dynamic-library bindings are not an
  alternate runtime path.
- Validated direct target service transitions are permitted only when the
  target contract is documented and stable for the supported OS versions.
- Undocumented or version-unstable service mechanisms are forbidden.
- Required signature, load-command, relocation, image-base, hardening, and
  explicit no-import metadata plus a direct Elderheim-owned serialization
  plan.
- Evidence that each selected service interface is stable enough for the
  supported OS/version contract; undocumented unstable service transitions are
  not accepted merely to avoid imports.
- Cross-field validation rejects impossible architecture/OS/format/ABI
  combinations in library APIs as well as the CLI.
- Windows x86_64, macOS AArch64, and all four Linux targets have explicit,
  non-interchangeable service contracts.

Verification:

- Exhaustive supported-target round trips pass.
- Forged and cross-target combinations fail before LIR construction.
- Capability snapshots are stable and contain no implicit host assumptions.
- Linux services cannot enter Windows/macOS plans and vice versa.
- Service-contract revision or content-fingerprint mismatch fails even when
  architecture, OS, and executable format otherwise match.
- Feasibility fixtures prove each target can represent entry, I/O, errors, and
  termination under the no-C/no-system-tool generated-output constraints.
- Any target-policy conflict blocks this stop and receives an explicit
  project-scope resolution release; import, console, signing, or loader policy
  cannot remain undecided or be relaxed implicitly in later PE/Mach-O
  milestones.

### v0.34.2 - Target-Parametric LIR And Services

Goal:

Ensure every validated LIR program is bound to exactly one supported target
without embedding platform-specific raw bytes.

Deliverables:

- `ValidatedLir<Target>` or an equivalent sealed target token carried through
  construction, validation, runtime selection, and backend dispatch.
- Replace `SysExit` and similar Unix-shaped operations with typed write, read,
  terminate, and failure service requests lowered under target capabilities.
- Typed symbols, labels, data regions, memory accesses, calls, and service
  signatures.
- A bounded transactional LIR builder with consuming validation.
- Validation of target agreement, types, control flow, symbol/data references,
  calling convention, stack discipline, and service availability.

Verification:

- Compile-fail and runtime tests reject handing one target's LIR to another
  backend.
- Invalid services, signatures, symbols, stack effects, and memory regions are
  rejected independently.
- Builder failure injection publishes no partial LIR.
- MIR and LIR interpreters agree on observable traces for lowered fixtures.

### v0.34.3 - Runtime Fragment Manifests And Isolation

Goal:

Make runtime selection, composition, and user/runtime isolation auditable before
fragments contain generated code.

Deliverables:

- A declarative manifest per fragment listing required/provided symbols,
  transitive dependencies, target services, code/data/scratch upper bounds,
  register clobbers, calling convention, stack requirements, failure behavior,
  return behavior, and accessible memory regions.
- Every manifest names the exact compatible `TargetServiceContractId` revision;
  broad target names are insufficient.
- Checked transitive closure with cycle, missing-provider, duplicate-provider,
  capability, target, and total-budget rejection.
- Reserved unforgeable runtime symbol IDs and separate read-only data,
  executable code, and writable-state regions.
- Calls only to declared entry points; no arbitrary relocation offsets,
  undeclared callbacks, or opaque embedded byte arrays.
- Fragment and service inventories attached to validated output reports.

Verification:

- Graph mutation tests cover cycles, missing/duplicate providers, incompatible
  targets, budget overflow, and undeclared services.
- Minimality tests prove no unneeded fragment is selected.
- Source symbols cannot alias or forge runtime symbols.
- Region, clobber, stack, callback, and entry-point contract violations fail
  before backend encoding.

### v0.34.4 - Architecture-Neutral Backend Contract

Goal:

Freeze a backend boundary that supports x86/x86_64 and AArch32/AArch64 without
forcing one architecture's registers, relocations, or service model onto the
others.

Deliverables:

- A sealed preparation trait consuming validated target LIR and producing a
  validated architecture-specific register/frame/machine-state plan.
- A sealed encoder trait consuming only that validated machine plan and
  producing a bounded encoded-region plan plus typed relocations and
  instruction-boundary entry tokens.
- Architecture-neutral lifecycle interfaces for `EncodedRegionPlan`, typed
  relocation records, symbols, and instruction-boundary tokens. Pre-encoder
  layout/writer stops use synthetic instances only; they do not claim real
  machine encoding.
- Architecture-owned register, operand, instruction, relocation, and immediate
  types behind the common lifecycle contract.
- No shared raw-byte, arbitrary-opcode, untyped patch-offset, generic register,
  or x86-specific RIP/rel32 abstraction.
- Common checked contracts for sections/regions, symbols, branch relaxation,
  relocation resolution, instruction boundaries, reports, and failure
  atomicity.
- Explicit AArch instruction alignment, scaled-immediate, literal-pool, branch,
  and veneer requirements represented before x86 implementation starts.

Verification:

- Mock x86 and AArch backends prove the common contract without sharing
  architecture-specific operands.
- Synthetic encoded-region fixtures prove layout APIs without bypassing the
  later machine-state and production encoder stops.
- Cross-architecture values cannot type-check at backend boundaries.
- Range, convergence, output-budget, and failure-atomicity contract tests pass.
- Backend reports bind target, architecture, ABI/services, regions, symbols,
  relocations, and emitted instruction count.

## Phase 5: Runtime Fragments

### v0.35.0 - Runtime Fragment Implementation

Goal:

Implement generated-program helpers without linking an external runtime, and
validate the complete user-plus-runtime LIR as one artifact.

Deliverables:

- Runtime fragment inventory.
- Fragment dependency graph.
- Fragment symbol naming.
- Fragment inclusion report.
- Derive runtime requirements only from validated MIR and the sealed target
  capability contract.
- Select and validate a bounded `RuntimePlan<Target>` before LIR construction.
- Lower the user program and every selected runtime fragment into one
  transactional target-parametric LIR builder.
- Run final LIR validation only after all user and runtime blocks, symbols,
  data, calls, stack effects, and service requests are present.
- No API appends fragments or mutates LIR after `ValidatedLir<Target>` exists.

Verification:

- Fragment selection tests.
- No-unused-fragment tests.
- Requirements, target capabilities, manifest closure, and runtime-plan reports
  agree exactly.
- Runtime requirements, plans, fragments, and final LIR all carry one identical
  service-contract ID.
- User-only, runtime-only, and cross-boundary malformed references fail final
  LIR validation.
- Failure during selection or either lowering path publishes no partial runtime
  plan or LIR.

### v0.36.0 - Write And Exit Runtime

Goal:

Provide minimal output and termination behavior.

Deliverables:

- `write_static` fragment contract.
- `exit` fragment contract.
- Platform output/exit ABI abstraction.
- Bounded partial-write and service-error handling under the portable runtime
  contract.
- Checked pointer-plus-length ranges and statically planned scratch state.
- Terminating error paths that cannot return into user code.

Verification:

- Runtime inventory for hello-world programs.
- Platform API report tests.
- Zero-progress, interrupted, partial, and failed service tests terminate within
  the configured retry bound.

### v0.37.0 - Historical Number Formatting Runtime

Goal:

Print edition-correct historical numeric values without libc or host math
libraries.

Deliverables:

- `print_number` implementing the frozen edition-selected BASIC 1, 2, and 4
  formatting contracts.
- Sign, spacing, significant-digit, exponent, rounding, and newline behavior.
- Fixed maximum buffer and checked pointer/length policy.
- Numeric domain/failure mapping to the portable runtime contract.

Verification:

- Historical-number formatting unit tests.
- Generated-output golden tests.
- Maximum-size, rounding-boundary, short-buffer, and cross-host deterministic
  output tests.

### v0.37.1 - Complete PRINT Layout Runtime

Goal:

Implement the stateful, edition-specific layout of complete PRINT statements,
not only isolated label or number formatting.

Deliverables:

- A `print_layout` runtime contract consuming typed label/number items and
  separator decisions from validated semantic lowering.
- Blank PRINT, mixed labels and historical numbers, numeric spacing,
  comma/zone positioning, trailing separators, and newline suppression.
- Manual-defined line wrapping or continuation behavior and persistent output
  column state across fragment calls.
- Sealed profile decisions for every PRINT behavior that changes across
  Dartmouth BASIC 1, 2, and 4.
- Checked column/zone arithmetic, bounded output chunks, and no hidden host
  terminal-width or locale dependency.
- Runtime inventory/report entries distinguish item formatting from statement
  layout state.

Verification:

- Exact complete output-byte fixtures cover blank PRINT, each separator,
  mixed labels/numbers, zone boundaries, trailing separators, wraps,
  continuation, and consecutive statements.
- Fragment-call partitioning cannot change the emitted byte stream or column
  state.
- Cross-edition fixtures prove profile-specific layout and rejection behavior.
- Column overflow, output-budget, partial-write, and state-corruption mutations
  fail deterministically and match the independent semantic oracle.

### v0.38.0 - Input Runtime

Goal:

Read and parse edition-correct historical numeric input without libc or host
math libraries.

Deliverables:

- `read_line`.
- `parse_number` using the frozen edition-selected BASIC 1, 2, and 4 historical
  numeric models.
- Input error exit path.
- Maximum input-line and numeric-token sizes.
- Bounded partial-read, EOF, delimiter, and service-error behavior.

Verification:

- stdin fixture tests.
- Invalid input tests.
- Oversized line/token, zero-progress, partial-read, EOF, exponent-boundary, and
  retry-limit tests.

### v0.38.1 - Complete INPUT Statement Runtime

Goal:

Implement complete manual-backed INPUT statement behavior for each supported
Dartmouth edition, not only line reading and isolated number parsing.

Deliverables:

- Edition-specific prompt text, prompt timing, output bytes, and interaction
  with current PRINT/output-column state.
- Multiple variables and fields with exact comma, whitespace, delimiter, and
  physical-line boundary behavior.
- Manual-backed handling of too few fields, surplus fields, empty fields,
  malformed fields, invalid-field retry/re-entry, and additional-line input.
- EOF and service failure behavior at statement start and after partial field
  consumption.
- A sealed per-edition assignment policy: all assignments commit atomically, or
  earlier variables remain assigned after later failure, exactly as the manual
  and errata ledger require.
- Bounded statement input state covering consumed bytes, parsed fields,
  pending assignments, prompts/retries, and resulting variable state.
- Exact INPUT trace reports: consumed input, emitted output, diagnostics,
  retries, assignments, and final state.

Verification:

- Manual-derived fixtures cover one/many variables, one/many lines, delimiter
  variants, too few/surplus/empty/invalid fields, retries, and EOF at every
  field boundary.
- Exact emitted bytes, consumed input, diagnostic, retry count, and resulting
  variable-state traces match the independent semantic oracle.
- Failure injection at every parse/assignment boundary proves the sealed
  edition-specific atomic-or-partial assignment rule.
- Oversized field counts, lines, tokens, retry loops, and pending-assignment
  state fail within configured budgets without leaking partial unchecked state.
- BASIC 1, 2, and 4 INPUT differences are explicit profile decisions; no modern
  BASIC behavior is inferred.

### v0.38.2 - Historical Numeric Operations Runtime

Goal:

Implement production generated-program arithmetic and comparisons with exactly
the historical model already proven by the independent reference oracle.

Deliverables:

- Runtime/LIR operations for add, subtract, multiply, divide, exponentiation,
  comparisons, conversions, `INT`, and `ABS` over `HistoricalNumber`.
- Sealed edition selection for every arithmetic, comparison, conversion,
  precision, and failure behavior that differs among BASIC 1, 2, and 4.
- Deterministic precision, rounding, signed-zero/applicability, overflow,
  underflow, division-by-zero, conversion, and comparison behavior.
- No host floating-point, host math library, libc, CPU-exception, or undefined
  instruction behavior defines source-language results.
- Checked bounded scratch/state requirements and fragment manifests for every
  production numeric operation.
- Explicit typed success/failure results that route through the portable
  runtime error contract and cannot return partially initialized numbers.

Verification:

- Production LIR/runtime traces match every independent numeric and semantic
  oracle vector exactly.
- Boundary and randomized deterministic vectors cover all arithmetic,
  comparison, conversion, overflow, underflow, and division failure paths.
- Failure injection publishes no partial LIR/runtime state.
- Cross-target instruction interpreters produce identical observable numeric
  traces without relying on native CPU traps.

### v0.38.3 - Numeric Functions And Deterministic RND Runtime

Goal:

Implement production generated-program functions and randomness with the same
deterministic semantics as the independent reference model.

Deliverables:

- Runtime/LIR implementations of `ATN`, `COS`, `EXP`, `LOG`, `SIN`, `SQR`, and
  `TAN`, plus any edition-ledger function delta not already covered at
  `v0.38.2`.
- Deterministic approximation constants, iteration bounds, range reduction,
  precision, and rounding without host math libraries.
- Production `RND` state, seed/reseed behavior, sequence, range, and
  repeatability under sealed edition rules.
- Domain/range/overflow/underflow failures mapped to the portable runtime error
  contract.
- Fixed scratch/state bounds and runtime manifests for every function and RND
  fragment.

Verification:

- Production LIR/runtime traces match all independent function and RND oracle
  vectors exactly across hosts and target interpreters.
- Domain boundaries, approximation transitions, iteration caps, seed edges,
  repeated calls, and state initialization are exhaustively or deterministically
  sampled under recorded bounds.
- Removing RND initialization or corrupting function scratch state fails
  validation or produces the edition-defined runtime error, never host behavior.
- No external math, randomness, native runtime, or CPU exception dependency is
  present in generated-binary reports.

### v0.39.0 - Data And Array Runtime

Goal:

Support DATA streams and arrays in generated binaries.

Deliverables:

- DATA cursor runtime.
- Array bounds runtime.
- Bounds failure exit path.
- Checked array-dimension multiplication, element-size multiplication, and total
  storage planning.
- `DATA` exhaustion and edition-specific `RESTORE` state semantics.
- FOR/NEXT and GOSUB/RETURN control-stack bounds, overflow, and underflow.
- A proof token that a dominating bounds check covers every array access.

Verification:

- DATA smoke tests.
- Array bounds tests.
- Dimension/storage overflow, DATA exhaustion/RESTORE, control-stack
  overflow/underflow, and missing-dominating-check tests.

### v0.39.1 - Executable Image Domain And Layout Planner

Goal:

Define format-independent checked image invariants before any ELF, PE, or
Mach-O serializer can publish bytes.

Deliverables:

- Non-interchangeable newtypes for file offsets/sizes, virtual addresses,
  memory sizes, alignments, table counts, section/segment IDs, and output size.
- One checked align-up operation rejecting zero, non-power-of-two alignment,
  addition overflow, and invalid file/virtual congruence.
- Private image-plan fields produced only by a bounded `LayoutPlanner`.
- Checked table multiplication, format-width narrowing, `u64`/`usize`
  conversion, file-size versus memory-size rules, and exact output budgets.
- Deterministic interval-sort overlap validation for headers, tables, sections,
  segments, code, data, and writable state.
- W^X permissions, target/class/machine/endianness agreement, and executable
  mapped-entry validation using an encoder-created instruction-boundary token.
- Layout consumes symbolic encoded regions and relocation metadata, assigns
  file/virtual ranges, and does not resolve patches before those addresses are
  final.

Verification:

- Every arithmetic, narrowing, alignment, overlap, permission, target, and
  entry invariant has limit-minus-one/limit/limit-plus-one tests.
- Segment-order permutations produce the same validated plan or stable error.
- Small-state models agree with alignment and interval-overlap validation.
- Invalid plans cannot be constructed through public APIs.

### v0.39.2 - Bounded Writers And Independent Image Verification

Goal:

Prove emitted image bytes exactly implement their validated plans before
format-specific writers are added.

Deliverables:

- A bounded output sink reserving against the planned exact file size.
- Writers accept only sealed regions whose typed relocations were resolved
  against the final validated layout.
- Format/endianness-typed field writers; every 1.0 target explicitly selects
  little-endian output, and Rust structs are never cast to bytes.
- Atomic writer completion: failure publishes no image and success requires the
  emitted byte count to equal the plan exactly.
- A deliberately independent image parser/verifier that reparses emitted bytes
  and compares target, format/class, headers, mapped ranges, permissions,
  entry point, and file/memory sizes with the plan.
- Deterministic image-verification staging and dependency reports. These
  reports do not create a publishable image capability before `v0.39.6`.

Verification:

- Synthetic format fixtures prove exact-size completion and failure atomicity.
- Mutation flips every modeled header field and perturbs every boundary by
  minus one, zero, and plus one; mismatches are rejected.
- Truncation, extension, overlap, permission, entry, class, machine, and
  endianness mutations fail independently.
- Repeated serialization produces byte-identical output and reports.

### v0.39.3 - Runtime Memory And Control-State Safety

Goal:

Close cross-fragment runtime safety invariants before generated runtime code is
combined with executable writers and native backends.

Deliverables:

- One checked pointer/range model for code, read-only data, mutable state,
  stack, arrays, DATA storage, input, output, and scratch buffers.
- Dominating bounds-check evidence for every runtime and user array access.
- Checked array dimension, stride, element-size, and total-storage arithmetic.
- Bounded FOR/NEXT and GOSUB/RETURN stacks with edition-correct overflow,
  underflow, and return behavior.
- Complete DATA exhaustion, cursor, and RESTORE state transitions.
- Bounded partial service loops with a progress invariant and hard iteration
  cap; no retry loop can be unbounded.
- Every runtime failure path terminates or transfers only to a declared typed
  handler and cannot accidentally return into user code.

Verification:

- Pointer/range and storage arithmetic are tested at every boundary and
  overflow point.
- Removing or moving a required bounds check invalidates the affected LIR.
- Stack, DATA, service-loop, scratch-buffer, and failure-return mutations each
  reach a stable rejection or runtime error.
- MIR, complete LIR, and independent semantic traces agree for all runtime
  success and failure fixtures.
- Complexity and maximum-memory reports remain within the unified compile and
  runtime budgets.

### v0.39.4 - Position Independence And Image Hardening Policy

Goal:

Freeze load-address and hardening behavior for every 1.0 format before the
first concrete executable writer is serialized.

Deliverables:

- A per-target fixed-address versus position-independent output decision.
- Preferred image base, permitted load bias, address-randomization contract,
  and report fields for every supported target.
- ELF executable type and load-bias policy, including whether secure ELF
  profiles are position independent.
- PE image-base, base-relocation metadata, NX/ASLR/control-flow hardening flags,
  no-import structure, and direct-service implications.
- Mach-O PIE, segment/load-command, relocation/rebase, and signature/load
  metadata policy.
- Tiny profiles are explicitly marked non-production and report every omitted
  hardening property; they are not the 1.0 supported release profile.
- Equivalent secure-profile requirements for ELF32/64, PE64, and Mach-O 64.

Verification:

- Cross-field target/format/image-base/load-bias combinations fail closed.
- Fixed and relocated synthetic plans reparse to the exact expected addresses
  and permissions.
- Generated-binary reports cannot omit position-independence, relocation,
  imports, signature/load metadata, NX, ASLR, or W^X status.
- Each 1.0 target has a feasible secure profile before format-specific work
  continues.

### v0.39.5 - Whole-Program Resource Certificate Contract

Goal:

Freeze a checked composition proof that bounds the complete generated program,
not only individual fragments, frames, arrays, or buffers.

Deliverables:

- A typed pre-serialization `ResourcePlan<Target>` bound to the exact opaque
  `CompilationIdentity`, `TargetServiceContractId`, validated runtime plan,
  LIR, machine plan, and image layout.
- A mandatory `ResourceCertificate<Target>` constructor contract. Independent
  image verification computes `image_digest = hash(executable_bytes)` and then
  computes a domain-separated certificate digest over the canonical resource
  plan, image digest, target-service contract fingerprint, and verifier version.
- Canonical resource-plan encoding plus fixed, versioned image/certificate
  digest algorithm identifiers, domain tags, and output widths. Algorithm or
  encoding changes invalidate existing certificates instead of being inferred.
- The certificate is immutable metadata carried beside executable bytes. It is
  not embedded in those bytes, so its image-digest binding cannot create a
  self-referential hashing cycle.
- Checked maximum native stack usage over the complete user/runtime call graph,
  including spill frames, saved registers, call-site alignment, service frames,
  and target ABI overhead.
- Checked maximum GOSUB and FOR/NEXT control-stack depth and storage.
- Runtime scratch, numeric/function/RND state, PRINT/INPUT state, arrays, DATA,
  input/output buffers, read-only static data, writable runtime memory, and
  total mapped image memory.
- Runtime-fragment dependency and call-graph SCC validation: dependencies are
  acyclic, or any permitted recursive call component has an explicit proven
  finite depth bound.
- Checked sum/product/alignment arithmetic with no double counting of shared
  buffers or fragments and no omission of target-specific state.
- A bounded composition validator and deterministic human/machine-readable
  certificate report. Pre-backend tests use synthetic machine/image plans; they
  do not claim final target certificates.
- Every later machine planner, backend, runtime, layout, and writer must
  contribute typed resource facts or fail certificate finalization.

Verification:

- Synthetic maximal call graphs, spill frames, control stacks, arrays, DATA,
  scratch, buffers, and image regions compose to exact expected bounds.
- Overflow, alignment, missing fact, duplicate/shared allocation, recursion,
  cycle, service-revision, target, and digest mismatches fail independently.
- Mutating any contributing plan, executable byte, service-contract
  fingerprint, compilation identity, or verifier version invalidates the
  certificate capability.
- Certificate generation is deterministic and bounded in time/memory.
- Independently derived known-answer vectors cover canonical resource plans
  with empty/minimum/maximum legal collections, lengths, frames, buffers, and
  image regions plus representative complete target plans.
- A strict independent resource-plan decoder rejects malformed lengths,
  alternate field order, duplicate/trailing/unknown fields, noncanonical
  versions/integers, missing compilation identity, and wrong identity domains
  without calling production canonicalization helpers.
- Final production completeness remains required at `v0.72.1`; this stop
  freezes the proof contract before backend implementation.

### v0.39.6 - Resource-Certified Verified Image Publication Boundary

Goal:

Make resource certification and independent image verification one mandatory
capability boundary that cannot be bypassed by the CLI, filesystem adapter,
tests, or future APIs.

Deliverables:

- Explicit lifecycle types: `ValidatedImageLayout`, `SealedRegions`, internal
  `SerializedImage`, validated `ResourcePlan<Target>`, mandatory
  `ResourceCertificate<Target>`, and externally publishable `VerifiedImage`.
- `SerializedImage` remains private staging data and cannot implement or reach
  user-output APIs.
- The independent parser/verifier consumes staging bytes plus the validated
  image and resource plans. It is the only constructor of `VerifiedImage` and
  must create and bind the final resource certificate in the same operation.
- `VerifiedImage` contains immutable executable bytes and certificate metadata
  as separate fields. There is no constructor, feature, test helper, or
  deserialization path for a certificate-free `VerifiedImage`.
- CLI/filesystem output adapters accept only `VerifiedImage` and publish through
  a same-directory temporary file plus atomic replacement where supported.
  Executable bytes are the program output; certificate/report artifacts remain
  separate metadata and never alter the hashed executable byte stream.
- Verification, certification, write, flush, permission, or replacement
  failure discards staging/temporary data and leaves any existing destination
  unchanged.
- Verified-image reports bind the exact executable digest, certificate digest,
  target, format, layout, permissions, entry point, relocations, imports,
  hardening, verifier version, `CompilationIdentity`, and
  `TargetServiceContractId` fingerprint.

Verification:

- Compile-fail API tests prove raw plans, sealed regions, serialized staging
  bytes, and uncertified verifier output cannot reach publication adapters.
- Constructor and deserialization tests prove a missing, empty, stale, forged,
  wrong-target, or wrong-image certificate cannot produce `VerifiedImage`.
- Verifier mutation tests prevent publication for every modeled image,
  resource-plan, certificate, and contract-fingerprint defect.
- Filesystem failure injection covers create, short write, flush, permission,
  rename/replace, and cleanup paths without corrupting an existing output.
- Successful executable publication bytes exactly match `image_digest`; any
  separate certificate/report artifact binds that digest without being part of
  its input.
- Checked-in known-answer vectors prove image and certificate domain separation
  for empty, minimum, maximum, and representative executable/resource inputs.
  Swapped domains, reused digests, malformed lengths, alternate field order,
  duplicate fields, trailing bytes, and noncanonical versions all fail under a
  strict implementation independent from production digest assembly.
- Repeated verification, certification, and publication are deterministic and
  do not weaken destination permissions.

## Phase 6: ELF Writers

### v0.40.0-elderheim - ELF Writer Core

Goal:

Build the common ELF serializer without unsafe header casting.

Deliverables:

- Explicit endian writers.
- ELF identification writer.
- Program header writer.
- Layout validation contracts.
- Independent ELF parser/verifier for every field emitted by this and later ELF
  stops; it shares no writer serialization helpers.

Verification:

- Exact-byte header tests.
- Invalid-layout tests.
- Writer/independent-parser plan agreement and header mutation tests.

### v0.41.0 - ELF64 Tiny Profile

Goal:

Write minimal ELF64 executables for early 64-bit targets.

Deliverables:

- ELF64 header.
- One PT_LOAD tiny profile.
- Entry point validation.
- No dynamic linker.
- Explicit non-production tiny-profile report, including omitted W^X,
  position-independence, stack, and load hardening.

Verification:

- ELF64 exact-byte tests.
- Linux loader smoke for x86_64 when backend exists.

### v0.42.0 - ELF32 Tiny Profile

Goal:

Write minimal ELF32 executables for early 32-bit targets.

Deliverables:

- ELF32 header.
- ELF32 program header.
- Entry point validation.
- No dynamic linker.
- Explicit non-production tiny-profile report, including omitted W^X,
  position-independence, stack, and load hardening.

Verification:

- ELF32 exact-byte tests.
- Linux loader smoke when 32-bit backend exists.

### v0.43.0 - Secure ELF64 Profile

Goal:

Add hardened ELF64 layout.

Deliverables:

- Separate R, R|X, R, and R|W segments.
- Non-executable stack metadata where supported.
- No RWX segment verification.
- Frozen ELF64 executable-type, image-base/load-bias, and
  position-independence policy from `v0.39.4`.

Verification:

- Segment permission tests.
- Generated binary report tests.

### v0.44.0 - Secure ELF32 Profile

Goal:

Add hardened ELF32 layout.

Deliverables:

- Separate segment layout for 32-bit targets.
- No RWX segment verification.
- 32-bit address-range checks.
- Frozen ELF32 executable-type, image-base/load-bias, and
  position-independence policy from `v0.39.4`.

Verification:

- ELF32 segment permission tests.
- Address overflow rejection tests.

### v0.44.1 - Arithmetic Guard And Failure-Lowering Contract

Goal:

Define typed arithmetic guards and failure edges before register allocation,
machine-state planning, instruction encoding, or relocation layout.

Deliverables:

- Guarded target-LIR contracts only for dynamic execution conditions: division
  by zero, signed division overflow, conversion range failures, dynamically
  computed invalid shift counts, and historical-number domain/range failures.
- Illegal instruction immediates, impossible static shift counts, unsupported
  operand widths, and unencodable constant forms are compiler validation
  errors. They never lower to runtime guards or runtime failure fragments.
- Architecture-neutral lowering templates route every modeled failure to the
  typed production numeric/runtime error path before a potentially trapping
  operation can execute.
- Guard/protected-operation identities, required dominance, success/failure
  edges, historical rule IDs, and non-returning failure semantics are explicit
  inputs to machine-state planning.
- The contract forbids architecture-specific trap/exception exposure unless an
  explicitly modeled and safely handled target contract permits it.
- Generated-binary reports must list guarded arithmetic operations and runtime
  failure fragments.

Verification:

- Target-LIR/interpreter fixtures exercise every success boundary and failure
  condition in the guard contract.
- Contract traces match the historical numeric and semantic oracles exactly for
  division, conversion, shifts, and function failures.
- Missing, duplicate, bypassed, or mismatched guard identities fail target-LIR
  validation before machine planning.
- Invalid static immediate/operand fixtures fail compilation before machine
  planning and cannot appear in runtime traces or generated-binary reports.
- Architecture-specific proof remains mandatory at `v0.46.1`, `v0.50.1`,
  `v0.54.1`, and `v0.58.1`; this contract stop is not backend completion.

### v0.44.2 - Register Allocation And Machine-State Validation

Goal:

Prove virtual values, arithmetic guards, stack state, flags, and runtime calls
form a valid target machine state before instruction encoding.

Deliverables:

- Per-target liveness analysis and virtual-register allocation, or a fully
  documented fixed-register lowering strategy where that is simpler and
  complete.
- Checked spill-slot allocation, alignment, non-overlap, and total frame-size
  calculation under compilation limits.
- Caller/callee-saved register sets, argument/result locations, shadow/red
  zones where applicable, call-site stack alignment, and return-state rules.
- Explicit condition/status flag definitions, uses, clobbers, and liveness.
- Runtime-fragment clobber and stack contracts enforced at every call site.
- A validator proving every physical register/flag use has a dominating
  definition and no live value is silently overwritten.
- Guard dominance and protected-operation ordering survive allocation, spills,
  block scheduling, flag use, and runtime failure transfer.
- Frozen minimum CPU baseline and permitted feature set for Linux x86/x86_64,
  Linux AArch32/AArch64, Windows x86_64, and macOS AArch64.

Verification:

- Liveness, interference, spill, frame, alignment, save/restore, flags, call,
  guard-dominance, and clobber defects each have independent rejection tests.
- Maximum-pressure and maximum-frame fixtures stay within documented time,
  memory, and stack bounds.
- Cross-target register classes, ABI states, and CPU features cannot enter the
  wrong backend.
- Independent machine-trace tests prove allocation preserves validated LIR and
  arithmetic failure behavior.
- Allocation and frame reports are deterministic across repeated runs.

### v0.44.3 - Typed Encoder And Relocation Security Contract

Goal:

Make illegal instruction/operand combinations and unsafe relocation patching
unrepresentable after machine-state preparation and before the first production
encoder subset is implemented.

Deliverables:

- Sealed architecture/mode encoders such as `Encoder<X86_64>`,
  `Encoder<X86_32>`, `Encoder<AArch64>`, and `Encoder<AArch32>`.
- Architecture-owned width-specific registers, immediates, relative offsets,
  address forms, and instruction-specific constructors.
- Explicit x86 ModRM/SIB and REX/high-byte restrictions plus AArch alignment,
  scaled-immediate, branch, literal-pool, and veneer invariants.
- Atomic instruction emission into a temporary maximum-instruction buffer
  before committing to the bounded encoded-program sink.
- Typed relocations binding kind, patch region/range, place address, target,
  addend, encoded width, architecture, and overflow policy.
- Non-overlapping patch ranges; each patch site belongs to its declared
  instruction/data field and is resolved exactly once.
- Required placeholder sentinels checked before patching.
- Branch/call targets restricted to instruction boundaries and data targets to
  their declared object ranges.
- Header, padding, and writable-storage targets rejected unless the relocation
  kind explicitly permits that destination class.
- Checked subtraction and explicit fit tests; no displacement truncation.
- An independent decoder/interpreter for the emitted subset and checked-in
  ISA-manual vector provenance.

Verification:

- Compile-fail tests reject wrong-width registers/immediates, cross-mode
  operands, invalid address forms, and untyped patch offsets.
- Exact manual vectors cover every legal operand class selected for the next
  encoder stop.
- `decode(encode(instruction)) == instruction` for exhaustive finite operand
  classes and bounded representative wide classes.
- Negative decoder tests reject truncation, trailing bytes, invalid/redundant
  prefixes, noncanonical encodings, wrong mode, and instruction-boundary
  confusion.
- Relocation field, placeholder, range, and sink-failure atomicity tests pass.
- No public production path emits arbitrary raw opcode bytes.

### v0.44.4 - Relaxation, Layout, And Relocation Sealing

Goal:

Resolve relocations only after symbolic encoded regions have a converged final
layout, then prevent any later address-changing mutation.

Deliverables:

- Encoding produces immutable symbolic regions, instruction boundaries,
  symbols, and unresolved typed relocations rather than final addresses.
- A bounded monotonic fixed-point loop coordinates branch relaxation, veneer or
  literal-pool decisions, region sizes, alignment, and provisional image
  layout.
- A hard iteration cap and progress metric; non-convergence is a structured
  compiler error with no partial image.
- Final file offsets and virtual addresses produce `ValidatedImageLayout`
  before relocation arithmetic begins.
- Checked relocation application verifies non-overlap, expected placeholder,
  field ownership, destination class/range, architecture width, and exact-once
  resolution.
- Resolution seals region sizes and bytes. Any later layout or byte mutation
  invalidates the capability and requires planning from symbolic input again.

Verification:

- Synthetic short/long branch and AArch veneer/literal-pool cases converge to
  stable layouts within the documented bound.
- Deliberate oscillation, non-progress, iteration overflow, displacement
  overflow, overlapping patches, duplicate resolution, sentinel mismatch, and
  invalid destination classes fail independently.
- Branch/call targets must be instruction boundaries; data targets must remain
  inside the declared object.
- Repeated planning produces identical layouts, patches, sealed bytes, and
  reports.
- The independent decoder and image reparser agree with every resolved field.

## Phase 7: x86 Backends

### v0.45.0 - x86_64 Encoder Core

Goal:

Encode the first x86_64 instruction subset safely.

Deliverables:

- Register model.
- REX prefix handling.
- Immediate moves.
- RIP-relative references.
- Typed Linux service-transition instruction lowering.
- Independently implemented x86_64 decoder/interpreter for every emitted
  instruction form.

Verification:

- Exact-byte instruction tests.
- Relocation placeholder tests.
- Encoder/decoder round trips and machine-trace agreement tests.
- Decoder rejection tests for truncation, trailing bytes, invalid/noncanonical
  encodings, wrong mode, and instruction-boundary confusion.

### v0.46.0 - x86_64 Relocations

Goal:

Patch x86_64 control-flow and data references safely.

Deliverables:

- RIP-relative data relocation.
- Branch rel32 relocation.
- Call rel32 relocation.
- Checked displacement math.

Verification:

- Relocation boundary tests.
- Out-of-range rejection tests.

### v0.46.1 - x86_64 Arithmetic Trap Equivalence

Goal:

Prove concrete x86_64 lowering cannot expose CPU arithmetic traps in place of
historical numeric semantics.

Deliverables:

- x86_64 guarded sequences for division by zero, signed division overflow,
  dynamic conversion range failures, dynamically computed invalid shift
  counts, and modeled domain/range failures.
- Illegal static immediates and operand forms are rejected by encoder
  validation and are never represented as runtime guard paths.
- Guard dominance, flags, register/clobber, and runtime-error transfer validated
  against the x86_64 machine-state plan.
- No uncontrolled `#DE`, invalid conversion, or exception-dependent result.

Verification:

- The independent x86_64 decoder/interpreter exercises every success and trap
  boundary and matches numeric/semantic oracle traces exactly.
- Guard removal, flag corruption, boundary inversion, and failure-return
  mutations are detected.
- No test exits through an uncontrolled CPU exception or OS signal path.

### v0.47.0 - Linux x86_64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable x86_64 Linux ELF64 binary.

Deliverables:

- x86_64 Linux service-transition lowering.
- ELF64 integration.
- CLI output path for one program.

Verification:

- In-process x86_64 instruction/image interpretation prints `HELLO` as the
  mandatory deterministic gate.
- Native Linux x86_64 execution is separate compatibility evidence.
- Report shows no dynamic linker and no external libraries.

### v0.48.0 - Linux x86_64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on x86_64.

Deliverables:

- LET/arithmetic output.
- GOTO/IF.
- FOR/NEXT.
- GOSUB/RETURN.

Verification:

- Generated fixture suite passes in the in-process x86_64 interpreter.
- Native Linux x86_64 execution is separate compatibility evidence.

### v0.49.0 - x86 32-bit Encoder Core

Goal:

Encode the first 32-bit x86 instruction subset.

Deliverables:

- 32-bit register model.
- Immediate moves.
- Relative branches.
- Typed Linux service-transition policy.
- Independently implemented x86 32-bit decoder/interpreter for every emitted
  instruction form.

Verification:

- Exact-byte instruction tests.
- ABI documentation tests.
- Encoder/decoder round trips and machine-trace agreement tests.
- Decoder rejection tests for truncation, trailing bytes, invalid/noncanonical
  encodings, wrong mode, and instruction-boundary confusion.

### v0.50.0-elderheim - x86 32-bit Relocations

Goal:

Patch 32-bit x86 control-flow and data references safely.

Deliverables:

- Branch relocation.
- Call relocation.
- Data address policy.
- Checked 32-bit address math.

Verification:

- Relocation boundary tests.
- Address overflow rejection tests.

### v0.50.1 - x86 32-bit Arithmetic Trap Equivalence

Goal:

Prove concrete x86 32-bit lowering cannot expose CPU arithmetic traps in place
of historical numeric semantics.

Deliverables:

- x86 32-bit guarded sequences for division by zero, signed division overflow,
  dynamic conversion range failures, dynamically computed invalid shift
  counts, and modeled domain/range failures.
- Illegal static immediates and operand forms are rejected by encoder
  validation and are never represented as runtime guard paths.
- Guard dominance, flags, register/clobber, stack, and runtime-error transfer
  validated against the x86 32-bit machine-state plan.
- No uncontrolled divide exception, invalid conversion, or
  exception-dependent result.

Verification:

- The independent x86 32-bit decoder/interpreter exercises every success and
  trap boundary and matches numeric/semantic oracle traces exactly.
- Guard removal, flag corruption, boundary inversion, and failure-return
  mutations are detected.
- No test exits through an uncontrolled CPU exception or OS signal path.

### v0.51.0 - Linux x86 32-bit Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable 32-bit x86 Linux ELF32 binary.

Deliverables:

- x86 Linux service-transition lowering.
- ELF32 integration.
- Generated binary report.

Verification:

- In-process x86 32-bit instruction/image interpretation prints `HELLO` as the
  mandatory deterministic gate.
- Native Linux x86 loader execution is recorded separately as compatibility
  evidence when suitable hardware/CI is available.

### v0.52.0 - Linux x86 32-bit Dartmouth Core

Goal:

Run core Dartmouth BASIC features on 32-bit x86.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes in the in-process x86 32-bit interpreter.
- Native Linux x86 execution is separate compatibility evidence.

## Phase 8: AArch Backends

### v0.53.0 - AArch64 Encoder Core

Goal:

Encode the first AArch64 instruction subset.

Deliverables:

- Register model.
- Immediate materialization policy.
- PC-relative data reference policy.
- Typed Linux service-transition convention.
- Independently implemented AArch64 decoder/interpreter for every emitted
  instruction form.

Verification:

- Exact-word instruction tests.
- ABI documentation tests.
- Encoder/decoder round trips and machine-trace agreement tests.
- Decoder rejection tests for truncation, trailing words, invalid/noncanonical
  encodings, wrong mode, and instruction-boundary confusion.

### v0.54.0 - AArch64 Relocations

Goal:

Patch AArch64 branches and data references safely.

Deliverables:

- Branch relocation.
- Call relocation.
- ADR/ADRP-style data reference policy.
- Checked immediate range math.

Verification:

- Relocation boundary tests.
- Out-of-range rejection tests.

### v0.54.1 - AArch64 Arithmetic Trap Equivalence

Goal:

Prove concrete AArch64 lowering cannot expose architectural arithmetic behavior
in place of historical numeric semantics.

Deliverables:

- AArch64 guarded sequences for division by zero, dynamic conversion range
  failures, dynamically computed invalid shift counts, and modeled domain/range
  failures.
- Illegal static immediates and operand forms are rejected by encoder
  validation and are never represented as runtime guard paths.
- Guard dominance, condition flags, register/clobber, stack, and runtime-error
  transfer validated against the AArch64 machine-state plan.
- Architectural non-trapping edge results are normalized to the historical
  model rather than leaking target-specific behavior.

Verification:

- The independent AArch64 decoder/interpreter exercises every success and
  failure boundary and matches numeric/semantic oracle traces exactly.
- Guard removal, condition corruption, boundary inversion, and failure-return
  mutations are detected.
- No test exposes an uncontrolled architectural exception or divergent
  target-specific arithmetic result.

### v0.55.0 - Linux AArch64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable AArch64 Linux ELF64 binary.

Deliverables:

- AArch64 Linux service-transition lowering.
- ELF64 integration.
- Generated binary report.

Verification:

- In-process AArch64 instruction/image interpretation prints `HELLO` as the
  mandatory deterministic gate.
- Native AArch64 Linux execution is separate compatibility evidence.

### v0.56.0 - Linux AArch64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on AArch64.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes in the in-process AArch64 interpreter.
- Native AArch64 Linux execution is separate compatibility evidence.

### v0.57.0 - AArch32 Encoder Core

Goal:

Encode the first 32-bit ARM/AArch32 instruction subset.

Deliverables:

- Register model.
- ARM/Thumb mode decision.
- Immediate materialization policy.
- Typed Linux service-transition convention.
- Independently implemented AArch32 decoder/interpreter for every emitted
  instruction form.

Verification:

- Exact-word instruction tests.
- ABI documentation tests.
- Encoder/decoder round trips and machine-trace agreement tests.
- Decoder rejection tests for truncation, trailing words, invalid/noncanonical
  encodings, wrong mode, and instruction-boundary confusion.

### v0.58.0 - AArch32 Relocations

Goal:

Patch AArch32 branches and data references safely.

Deliverables:

- Branch relocation.
- Call relocation.
- Literal/data reference policy.
- Checked immediate range math.

Verification:

- Relocation boundary tests.
- Out-of-range rejection tests.

### v0.58.1 - AArch32 Arithmetic Trap Equivalence

Goal:

Prove concrete AArch32 lowering cannot expose architectural arithmetic behavior
in place of historical numeric semantics.

Deliverables:

- AArch32/selected-mode guarded sequences for division by zero, dynamic
  conversion range failures, dynamically computed invalid shift counts, and
  modeled domain/range failures.
- Illegal static immediates and operand forms are rejected by encoder
  validation and are never represented as runtime guard paths.
- Guard dominance, condition flags, register/clobber, stack, and runtime-error
  transfer validated against the AArch32 machine-state plan.
- Architecture/mode-specific edge results are normalized to the historical
  model rather than leaking target behavior.

Verification:

- The independent AArch32 decoder/interpreter exercises every success and
  failure boundary and matches numeric/semantic oracle traces exactly.
- Guard removal, condition corruption, boundary inversion, mode confusion, and
  failure-return mutations are detected.
- No test exposes an uncontrolled architectural exception or divergent
  target-specific arithmetic result.

### v0.59.0 - Linux AArch32 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable AArch32 Linux ELF32 binary.

Deliverables:

- AArch32 Linux service-transition lowering.
- ELF32 integration.
- Generated binary report.

Verification:

- In-process AArch32 instruction/image interpretation prints `HELLO` as the
  mandatory deterministic gate.
- Native AArch32 Linux execution is separate compatibility evidence.

### v0.60.0-elderheim - Linux AArch32 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on AArch32.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes in the in-process AArch32 interpreter.
- Native AArch32 Linux execution is separate compatibility evidence.

## Phase 9: Windows And Mac Output

### v0.61.0 - PE Writer Core

Goal:

Build the common PE/COFF serializer needed for Windows output.

Deliverables:

- Explicit little-endian PE writers.
- DOS stub policy.
- COFF header writer.
- Optional-header writer.
- Section table writer.
- PE layout verifier.
- Independent PE/COFF parser/verifier for every emitted header, directory, and
  section field; it shares no writer serialization helpers.

Verification:

- Exact-byte PE header tests.
- Invalid-layout tests.
- Writer/independent-parser plan agreement and field mutation tests.

### v0.62.0 - PE64 Tiny Profile

Goal:

Write minimal PE64 executables for Windows x86_64.

Deliverables:

- PE32+ optional header.
- `.text`, `.rdata`, `.data`, and `.bss` section layout.
- Entry point validation.
- No import directory or dynamic OS-library binding, as frozen at `v0.13.7`.
- No external BASIC runtime.
- Explicit non-production tiny-profile report listing omitted base relocation,
  ASLR, NX, and other hardened-image properties.

Verification:

- PE64 exact-byte tests.
- PE layout report tests.

### v0.62.1 - Secure PE64 Profile

Goal:

Implement the production Windows x86_64 image-hardening contract before ABI
and runtime integration.

Deliverables:

- Frozen preferred image base and complete base-relocation metadata when the
  approved policy permits rebasing.
- Approved ASLR, NX, high-entropy address, control-flow, and terminal-server
  flags where applicable to the supported Windows contract.
- No import/service directory or dynamic OS-library binding; section
  permissions exactly match the `v0.34.1` feasibility decision.
- No RWX section, executable writable data, import, or relocation into
  headers/padding.
- Independent parser verification of every security-relevant optional-header
  flag, directory, section, absence of imports, and base-relocation block.

Verification:

- Exact-byte secure PE fixtures and generated-binary reports pass.
- Rebase simulations and relocation mutations prove exact-once bounded patching
  at permitted destinations.
- Missing/contradictory hardening flags, injected imports, RWX permissions,
  malformed directories, and image-base overflow fail independently.
- The secure profile, not the tiny profile, is required by Windows 1.0 fixture
  and release gates.

### v0.63.0 - Windows x86_64 ABI Lowering

Goal:

Lower Elderheim LIR/runtime calls into the Windows x86_64 ABI.

Deliverables:

- Windows x64 calling convention model.
- Stack alignment and shadow-space policy.
- Implement the frozen process-entry and exit service contracts.
- Implement the frozen console output and input service contracts.

Verification:

- ABI lowering unit tests.
- Runtime-fragment inventory for Windows output.

### v0.64.0 - Windows x86_64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a Windows x86_64 PE64 executable.

Deliverables:

- PE64 integration.
- Windows output path in CLI internals.
- Generated binary report for Windows.

Verification:

- In-process Windows x86_64 instruction/PE interpretation prints `HELLO` as the
  mandatory deterministic gate.
- Native Windows 11 and Windows Server 2025 execution is recorded separately as
  compatibility evidence.

### v0.65.0 - Windows x86_64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on Windows x86_64.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.
- Runtime error exit paths.

Verification:

- Generated fixture suite passes in the in-process Windows x86_64/PE
  interpreter.
- Native Windows x86_64 execution is separate compatibility evidence.

### v0.66.0 - Mach-O Writer Core

Goal:

Build the common Mach-O serializer needed for macOS output.

Deliverables:

- Mach-O 64-bit header writer.
- Load-command writer.
- Segment/section layout model.
- Entry point command policy.
- The frozen `v0.34.1` signature/load-metadata policy.
- Independent Mach-O parser/verifier for every emitted header, command,
  segment, section, relocation/rebase, and signature/load-metadata field.

Verification:

- Exact-byte Mach-O header tests.
- Invalid-layout tests.
- Writer/independent-parser plan agreement and field mutation tests.

### v0.67.0 - Mach-O AArch64 Tiny Profile

Goal:

Write minimal Mach-O 64-bit executables for Apple Silicon macOS.

Deliverables:

- `arm64` Mach-O header constants.
- `__TEXT`, `__DATA`, and `__LINKEDIT` layout policy.
- Entry point validation.
- Page alignment policy.
- Explicit non-production tiny-profile report listing omitted PIE, rebase,
  signature/load, and other hardened-image properties.

Verification:

- Mach-O layout tests.
- Generated binary report tests.

### v0.67.1 - Secure Mach-O AArch64 Profile

Goal:

Implement the production Apple Silicon image-hardening and loader-metadata
contract before ABI and runtime integration.

Deliverables:

- Position-independent Mach-O layout with the frozen preferred base/load-bias,
  PIE, segment, relocation/rebase, and load-command policy.
- Separate non-writable executable code, read-only data, and writable state;
  no RWX segment or executable writable section.
- Direct Elderheim serialization of every required signature/ad-hoc-signature
  or load metadata selected by the `v0.34.1` feasibility gate.
- Entry point at a verified instruction boundary inside mapped executable
  `__TEXT`.
- Independent parser verification of all security-relevant commands, segments,
  sections, rebases/relocations, entry metadata, and signature/load data.

Verification:

- Exact-byte secure Mach-O fixtures and generated-binary reports pass.
- Load-bias/rebase simulations preserve declared references and permissions.
- Missing/contradictory PIE or load metadata, malformed commands, overlap, RWX,
  invalid entry points, and signature-range defects fail independently.
- The secure profile, not the tiny profile, is required by macOS 1.0 fixture
  and release gates.

### v0.68.0 - macOS AArch64 ABI Lowering

Goal:

Lower Elderheim LIR/runtime calls into the macOS Apple Silicon ABI.

Deliverables:

- AArch64 Darwin calling convention model.
- Stack alignment policy.
- Implement the frozen process-entry and exit service contracts.
- Implement the frozen console output and input service contracts.

Verification:

- ABI lowering unit tests.
- Runtime-fragment inventory for macOS output.

### v0.69.0 - macOS AArch64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a macOS Apple Silicon Mach-O executable.

Deliverables:

- Mach-O integration.
- macOS output path in CLI internals.
- Generated binary report for macOS.

Verification:

- In-process AArch64/Mach-O interpretation prints `HELLO` as the mandatory
  deterministic gate.
- Native Apple Silicon macOS execution is separate compatibility evidence.

### v0.70.0-elderheim - macOS AArch64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on Apple Silicon macOS.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.
- Runtime error exit paths.

Verification:

- Generated fixture suite passes in the in-process AArch64/Mach-O interpreter.
- Native Apple Silicon macOS execution is separate compatibility evidence.

### v0.71.0 - Cross-Platform Runtime Conformance

Goal:

Prove Linux, Windows, and macOS implementations conform to the observable
runtime contract frozen at `v0.18.1`.

Deliverables:

- Final target service/API inventory.
- Exit-code, newline, encoding, EOF, partial-service, and runtime-error
  conformance matrix.
- Explicit target deviations with compatibility and generated-binary report
  evidence.

Verification:

- Runtime report tests for all 1.0 platforms.
- Output-equivalence tests for portable programs.
- Undeclared platform deviations fail the conformance gate.

### v0.72.0 - Cross-Platform Output Matrix

Goal:

Prove that all 1.0 output formats are represented before CLI/report work.

Deliverables:

- Linux ELF32/ELF64 matrix.
- Windows PE64 matrix.
- macOS Mach-O aarch64 matrix.
- Native compatibility-evidence and optional emulator policy document; neither
  is an implementation dependency.
- Known platform limitation list.

Verification:

- Target matrix tests.
- Format matrix tests.
- Documentation link checks.

### v0.72.1 - Cross-Target Resource Certificate Matrix

Goal:

Finalize and verify whole-program resource certificates for every supported
language profile, target, runtime composition, and generated secure image.

Deliverables:

- Final certificates for Dartmouth BASIC 1, 2, and 4 across Linux x86,
  x86_64, AArch32, AArch64, Windows x86_64, and macOS AArch64.
- Certificates bind exact semantic contract, compilation identity,
  service-contract revision/fingerprint, runtime fragments, call graph, machine
  frames, spills, control stacks, arrays/DATA, scratch/buffers, image regions,
  writable memory, verified-image digest, and hardening profile.
- Maximum and representative manual-derived programs for each profile/target.
- Cross-target comparison reports explain ABI/frame/image differences without
  changing portable language/runtime bounds.
- No synthetic, incomplete, tiny-profile, stale-revision, or unverified-image
  certificate is accepted as production evidence.

Verification:

- Every fixture/target matrix entry produces and validates one complete
  deterministic certificate.
- Independently recomputed stack, memory, call-depth, and image totals agree
  with each certificate.
- Removing any fragment, frame, buffer, region, revision, or digest fact fails
  the matrix gate.
- Substituting source, edition, semantics, options, limits, or schema identity
  from another otherwise-valid matrix entry fails independently.
- Maximum-bound fixtures remain within documented compiler/runtime/image limits.
- Native compatibility evidence is linked to the exact verified-image digest
  and certificate without becoming a compiler implementation dependency.

## Phase 10: CLI, Reports, And User Experience

### v0.73.0 - CLI Target Selection

Goal:

Make target selection stable and testable.

Deliverables:

- `--target linux-x86-elf32`.
- `--target linux-x86_64-elf64`.
- `--target linux-aarch32-elf32`.
- `--target linux-aarch64-elf64`.
- `--target windows-x86_64-pe64`.
- `--target macos-aarch64-macho64`.
- Default target policy.
- Output path policy.

Verification:

- CLI parser tests.
- Unsupported target diagnostics.

### v0.74.0 - Dartmouth Version CLI

Goal:

Make Dartmouth version selection stable and explicit.

Deliverables:

- `--dartmouth-basic-1`.
- `--dartmouth-basic-2`.
- `--dartmouth-basic-4`.
- No `--dartmouth-basic-3`.
- `--dialect` alias policy if added.

Verification:

- CLI parser tests.
- Version 3 rejection diagnostic.

### v0.75.0 - Compatibility Reports

Goal:

Report source compatibility clearly.

Deliverables:

- Version report.
- Semantic-contract and compilation-identity report.
- Feature usage report.
- Unsupported-feature report.
- Control-flow report.

Verification:

- Golden report tests.

### v0.76.0 - Generated Binary Reports

Goal:

Make executable output auditable.

Deliverables:

- Target report.
- Format report.
- Compilation identity, target-service fingerprint, executable digest, and
  resource-certificate digest report.
- Syscall report.
- Runtime fragment report.
- Dynamic dependency report.

Verification:

- Golden report tests for all 1.0 targets.

### v0.77.0 - Error Message Polish

Goal:

Make diagnostics useful for real users.

Deliverables:

- Stable diagnostic codes.
- Source snippets.
- Version-specific hints.
- Target-specific hints.

Verification:

- Diagnostic golden tests.
- No panic on malformed inputs.

## Phase 11: Compatibility And Conformance

### v0.78.0 - Dartmouth BASIC 1 Compatibility Sweep

Goal:

Close known version 1 compatibility gaps.

Deliverables:

- Version 1 feature matrix.
- Version 1 fixture suite.
- Completion-blocker register with every committed-scope item fixed or assigned
  to an explicit follow-up release.

Verification:

- Version 1 fixtures pass on all implemented targets.

### v0.79.0 - Dartmouth BASIC 2 Compatibility Sweep

Goal:

Close known version 2 compatibility gaps.

Deliverables:

- Version 2 feature matrix.
- Version 2 fixture suite.
- Completion-blocker register with every committed-scope item fixed or assigned
  to an explicit follow-up release.

Verification:

- Version 2 fixtures pass on all implemented targets.

### v0.80.0-elderheim - Dartmouth BASIC 4 Compatibility Sweep

Goal:

Close known version 4 compatibility gaps.

Deliverables:

- Version 4 feature matrix.
- Version 4 fixture suite.
- Completion-blocker register with every committed-scope item fixed or assigned
  to an explicit follow-up release.

Verification:

- Version 4 fixtures pass on all implemented targets.

### v0.81.0 - Cross-Version Compatibility Matrix

Goal:

Prove that version-specific behavior is intentional.

Deliverables:

- Cross-version fixture matrix.
- Accepted/rejected feature table.
- Version migration notes.

Verification:

- Cross-version matrix tests.
- Documentation link checks.

### v0.82.0 - Cross-Target Output Matrix

Goal:

Prove that supported programs behave the same across targets.

Deliverables:

- In-process instruction/image fixture runner for Linux x86, Linux x86_64,
  Linux AArch32, Linux AArch64, Windows x86_64, and macOS AArch64.
- Output comparison.
- Exit-code comparison.
- Runtime-fragment comparison.
- Separately recorded native compatibility evidence for every target support
  claim; emulator evidence is labeled and cannot replace native evidence at
  final 1.0 acceptance.

Verification:

- Cross-target fixture suite passes.
- Native evidence is reviewed separately without entering compiler execution or
  deterministic oracle paths.

### v0.82.1 - Independent Semantic And Output Oracles

Goal:

Establish independent agreement across source semantics, MIR, emitted
instructions, and executable images before the cumulative security campaign.

Deliverables:

- A pure Rust Dartmouth semantic interpreter implemented independently from
  production lowering and runtime fragments.
- An independent MIR interpreter and emitted-instruction-subset
  decoder/interpreter.
- An independent ELF/PE/Mach-O image reparser using no writer implementation
  helpers.
- A common observable trace covering output bytes, input consumption, runtime
  failure, memory-region accesses, service calls, and exit status.
- Manual-derived fixture metadata carrying edition, source/manual identifier,
  page or rule reference, expected result, and expected rejection mode.

Verification:

- Semantic HIR, MIR, and in-process LIR/instruction target traces agree for
  every applicable fixture; native evidence is compared separately.
- Deliberate defects in each production layer are detected by at least one
  independent oracle test.
- Oracle implementations share data formats only, not validator, lowerer,
  encoder, runtime, or writer logic.
- Cross-target traces agree except for explicitly reported ABI/service details.

### v0.82.2 - Mutation, Property, And Small-State Models

Goal:

Run deterministic high-assurance campaigns across every compiler trust
boundary before final focused abuse sweeps.

Deliverables:

- Deterministic byte mutation for normalization, line tables, lexers, parsers,
  and diagnostics, including all 256 byte values, line-ending corruption,
  digit/quote storms, controls, bidi text, nesting, and budget edges.
- Raw MIR/LIR generation under configured limits plus mutation of valid
  programs so each invariant is independently exercised.
- Exhaustive legal finite encoder operand classes and deterministic sampled
  wide classes, with encoder/decoder round trips.
- Relocation and executable-layout boundary mutation at minus one, zero, and
  plus one around every range and width limit.
- Small-state executable models for definitions/references, CFG termination,
  dominance, relocation arithmetic, alignment, and interval non-overlap.
- Reproducibility properties for diagnostics, reports, IR, encoded bytes, and
  executable images.

Verification:

- Fixed-seed campaigns are reproducible and emit bounded failure artifacts.
- Every public parser/validator/builder accepts deterministically or returns a
  structured error without panic.
- No rejected input produces later-stage IR or output.
- Model/implementation comparisons pass for every exhaustively enumerated
  small state.

## Phase 12: Security Hardening

### v0.83.0 - Parser Abuse Hardening

Goal:

Run the final cumulative source-ingestion and parser campaign after incremental
hardening at every frontend stop.

Deliverables:

- Malformed source corpus.
- Deep expression limits.
- Huge line-number tests.
- Diagnostic flood limits.
- Absolute-span/source-identity mutation.
- Cross-edition profile confusion tests.
- Normalized-source capability bypass checks.

Verification:

- Abuse tests pass.
- No panics.

### v0.84.0 - Relocation And Layout Hardening

Goal:

Run the final cumulative instruction, relocation, and executable-layout
campaign after incremental backend/writer hardening.

Deliverables:

- Relocation fuzz-style unit tests.
- Address overflow tests.
- Segment overlap tests.
- Entry-point validation tests.
- Encoder/decoder differential vectors.
- Cross-target, class, ABI, and service confusion tests.

Verification:

- Encoder, relocation, and layout abuse tests pass for ELF32, ELF64, PE64, and
  Mach-O 64 on their supported architectures.

### v0.85.0 - Runtime Fragment Hardening

Goal:

Run the final cumulative generated-program runtime campaign after incremental
fragment tests at every runtime stop.

Deliverables:

- Input bounds tests.
- Array bounds tests.
- Data cursor bounds tests.
- Division failure tests.
- Fragment graph, symbol isolation, stack/clobber, service, and memory-region
  contract mutation.

Verification:

- Runtime abuse fixtures pass on all 1.0 targets.

### v0.86.0 - Dependency And Tooling Gate

Goal:

Make release tooling stricter before 1.0 candidates.

Deliverables:

- `cargo deny` gate.
- `cargo audit` gate.
- SBOM generation gate.
- Modularity gate.
- Release readiness gate.

Verification:

- Local release gate passes.

### v0.87.0 - Reproducible Output Check

Goal:

Prove deterministic generated binaries for fixed inputs.

Deliverables:

- Same input produces same output bytes.
- Build metadata policy.
- Timestamp policy.
- Stable output ordering.

Verification:

- Reproducible generated-output tests.

## Phase 13: 1.0 Release Candidates

### v0.88.0 - Documentation Freeze Candidate

Goal:

Freeze the user-visible 1.0 contract.

Deliverables:

- README complete for 1.0 scope.
- CLI reference.
- Dartmouth version reference.
- Target support matrix.
- Security model.
- Scope exclusions and completion-blocker register.

Verification:

- Documentation link check.
- Example commands tested.

### v0.89.0 - Feature Freeze Candidate

Goal:

Stop adding features and focus on correctness.

Deliverables:

- No new language features after this tag without resetting release-candidate
  status.
- Fixture matrix complete.
- Target matrix complete.
- Reports complete.

Verification:

- Full fixture matrix passes.

### v0.90.0-elderheim - Performance And Size Baseline

Goal:

Record realistic compiler and generated-output baselines.

Deliverables:

- Compile-time baseline.
- Generated binary size baseline.
- Runtime fragment size report.
- Large-program behavior notes.

Verification:

- Baseline script passes.
- Regressions documented.

### v0.91.0 - Security Review Candidate

Goal:

Prepare the exact commit for security review.

Deliverables:

- Threat model updated.
- Unsafe policy updated.
- Supply-chain evidence updated.
- Pentest handoff notes.

Verification:

- `scripts/checks.sh`
- `cargo deny check`
- `cargo audit`
- `scripts/generate-sbom.sh`

### v0.92.0 - Pentest Fix Candidate

Goal:

Address findings from the release-candidate security review.

Deliverables:

- Fixes for release-blocking findings.
- Regression tests for every fixed finding.
- Updated security notes.

Verification:

- Follow-up pentest review passes or every non-blocking finding is assigned to
  an explicit follow-up release.

### v0.93.0 - 1.0 Final Candidate

Goal:

Produce the final pre-1.0 candidate.

Deliverables:

- All 1.0 docs complete.
- All release notes complete.
- SBOM generated.
- Pentest report ready for final pass.
- No known release-blocking issues.

Verification:

- Full local gate.
- Full fixture matrix.
- Full target matrix.

## Phase 14: 1.0

### v1.0.0 - Dartmouth BASIC 1, 2, And 4 Compiler

Goal:

Ship the first serious production-ready language milestone for the Elderheim
compiler platform, using Dartmouth BASIC as the proving implementation.

Required support:

- Dartmouth BASIC version 1 profile.
- Dartmouth BASIC version 2 profile.
- Dartmouth BASIC version 4 profile.
- Dartmouth BASIC version 3 remains unsupported until official documentation is
  available.
- Linux x86 ELF32 output.
- Linux x86_64 ELF64 output.
- Linux AArch32 ELF32 output.
- Linux AArch64 ELF64 output.
- Windows x86_64 PE64 output.
- macOS Apple Silicon AArch64 Mach-O output.
- Secure production ELF32/ELF64, PE64, and Mach-O 64 profiles; tiny profiles are
  test scaffolds and are not accepted as 1.0 release output.
- No external backend, assembler, linker, libc, or BASIC runtime dependency for
  generated programs.
- Observable runtime behavior conforms across targets or reports an explicitly
  approved target deviation.
- Independent semantic, MIR, instruction, and image oracles agree with the full
  fixture and target matrices.
- Compatibility reports.
- Generated-binary reports.
- Security and supply-chain release evidence.

Verification:

- `scripts/checks.sh`.
- `cargo deny check`.
- `cargo audit`.
- `scripts/generate-sbom.sh`.
- Full Dartmouth version fixture matrix.
- Full Linux, Windows, and macOS target fixture matrix.
- Native execution compatibility evidence for every claimed 1.0 target;
  emulator-only evidence is insufficient for final support acceptance.
- Pentest report status `PASS`.

Post-1.0 directions:

- Other documented BASIC variants.
- Windows 32-bit output, if it becomes useful enough to justify the work.
- Intel macOS output, only if there is a clear support requirement.
- BSD and Android target hardening.
- Other older language-family crates only after source material and scope are
  ready.
- Aesynx output path when Aesynx is ready.
