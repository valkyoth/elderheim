# Elderheim Release Plan To 1.0

Status: planning document

This plan is intentionally granular. Elderheim is not a small parser project:
it is a direct native compiler, executable writer, runtime-fragment system, and
compatibility project for Dartmouth BASIC versions 1, 2, and 4.

`v1.0.0` is the first serious production-ready language milestone for the
Elderheim compiler platform. It proves the platform with Dartmouth BASIC. It is
not the end of Elderheim.

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
- Compatibility reports.
- Generated-binary dependency reports.
- Security and release evidence.

Windows support is intentionally scoped to `x86_64` for 1.0. macOS support is
intentionally scoped to Apple Silicon `aarch64` for 1.0.

## Version Sequencing Rule

Dartmouth BASIC versions must be implemented in order:

1. BASIC 1 reaches a complete supported subset.
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
- No hidden dependency on a developer machine.
- Documentation of known limitations.
- No source file over 500 lines.

Tag readiness requires:

- `scripts/checks.sh` passes.
- `cargo deny check` passes.
- `cargo audit` passes.
- `scripts/generate-sbom.sh` succeeds.
- Release notes exist under `release-notes/`.
- A pentest report exists under `security/pentest/`.
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

Use `-release` suffixed tags for those exact milestones:

- `v0.10.0-release`
- `v0.20.0-release`
- `v0.30.0-release`
- `v0.40.0-release`
- `v0.50.0-release`
- `v0.60.0-release`
- `v0.70.0-release`
- `v0.80.0-release`
- `v0.90.0-release`

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
  host model, and compatibility fixtures.
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
| v0.10.0-release | BASIC 1 line table behavior is stable. | P2 |
| v0.11.0 | BASIC 1 lexer fixtures pass. | P2 |
| v0.12.0 | BASIC 1 HIR snapshots are stable. | P2 |
| v0.13.0 | BASIC 1 minimal parser fixtures pass. | P2 |
| v0.14.0 | BASIC 1 variables, numbers, and expressions parse correctly. | P2 |
| v0.15.0 | BASIC 1 control-flow parser rejects later-version syntax. | P2 |
| v0.16.0 | BASIC 1 semantic validation and CFG reports pass. | P2 |
| v0.17.0 | BASIC 1 MIR golden tests pass. | P2 |
| v0.18.0 | BASIC 1 runtime requirements are fully inventoried. | P3 |
| v0.19.0 | BASIC 1 host model executes semantic fixtures. | P2 |
| v0.20.0-release | BASIC 1 compatibility sweep passes before BASIC 2 starts. | P2 |
| v0.21.0 | BASIC 2 delta is documented and BASIC 1 remains green. | P2 |
| v0.22.0 | BASIC 2 lexer delta passes and BASIC 1 rejects BASIC 2-only tokens. | P2 |
| v0.23.0 | BASIC 2 parser delta passes without BASIC 1 regressions. | P2 |
| v0.24.0 | BASIC 2 semantic validation passes without BASIC 1 regressions. | P2 |
| v0.25.0 | BASIC 2 MIR/runtime delta passes without BASIC 1 regressions. | P2 |
| v0.26.0 | BASIC 2 host model executes semantic fixtures. | P2 |
| v0.27.0 | BASIC 2 compatibility sweep passes before BASIC 4 starts. | P2 |
| v0.28.0 | BASIC 4 delta is documented and older versions remain green. | P2 |
| v0.29.0 | BASIC 4 lexer delta passes and older modes reject BASIC 4-only tokens. | P2 |
| v0.30.0-release | BASIC 4 parser delta passes without older-version regressions. | P2 |
| v0.31.0 | BASIC 4 semantic validation passes without older-version regressions. | P2 |
| v0.32.0 | BASIC 4 MIR/runtime delta passes without older-version regressions. | P2 |
| v0.33.0 | BASIC 4 host model executes semantic fixtures. | P2 |
| v0.34.0 | BASIC 1, 2, and 4 compatibility sweeps all pass. | P2 |
| v0.35.0 | Runtime fragments lower into target-near LIR with inventory reports. | P3 |
| v0.36.0 | Write and exit runtime behavior is platform-abstracted. | P3 |
| v0.37.0 | Integer formatting runtime passes bounds and golden tests. | P3 |
| v0.38.0 | Input runtime passes valid and invalid input tests. | P3 |
| v0.39.0 | DATA and array runtime passes bounds tests. | P3 |
| v0.40.0-release | ELF writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.41.0 | ELF64 tiny profile is layout-verified. | P4 |
| v0.42.0 | ELF32 tiny profile is layout-verified. | P4 |
| v0.43.0 | Secure ELF64 profile enforces segment permissions. | P4 |
| v0.44.0 | Secure ELF32 profile enforces segment permissions and address bounds. | P4 |
| v0.45.0 | x86_64 encoder exact-byte tests pass. | P5 |
| v0.46.0 | x86_64 relocation boundary tests pass. | P5 |
| v0.47.0 | Linux x86_64 hello-world binary smoke passes. | P5 |
| v0.48.0 | Linux x86_64 Dartmouth core fixture suite passes. | P5 |
| v0.49.0 | x86 32-bit encoder exact-byte tests pass. | P5 |
| v0.50.0-release | x86 32-bit relocation boundary tests pass. | P5 |
| v0.51.0 | Linux x86 32-bit hello-world binary smoke passes. | P5 |
| v0.52.0 | Linux x86 32-bit Dartmouth core fixture suite passes. | P5 |
| v0.53.0 | AArch64 encoder exact-word tests pass. | P5 |
| v0.54.0 | AArch64 relocation boundary tests pass. | P5 |
| v0.55.0 | Linux AArch64 hello-world binary smoke passes. | P5 |
| v0.56.0 | Linux AArch64 Dartmouth core fixture suite passes. | P5 |
| v0.57.0 | AArch32 encoder exact-word tests pass. | P5 |
| v0.58.0 | AArch32 relocation boundary tests pass. | P5 |
| v0.59.0 | Linux AArch32 hello-world binary smoke passes. | P5 |
| v0.60.0-release | Linux AArch32 Dartmouth core fixture suite passes. | P5 |
| v0.61.0 | PE writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.62.0 | PE64 tiny profile is layout-verified. | P4 |
| v0.63.0 | Windows x86_64 ABI lowering tests pass. | P5 |
| v0.64.0 | Windows x86_64 hello-world binary smoke passes. | P5 |
| v0.65.0 | Windows x86_64 Dartmouth core fixture suite passes. | P5 |
| v0.66.0 | Mach-O writer core passes exact-byte and invalid-layout tests. | P4 |
| v0.67.0 | Mach-O AArch64 tiny profile is layout-verified. | P4 |
| v0.68.0 | macOS AArch64 ABI lowering tests pass. | P5 |
| v0.69.0 | macOS AArch64 hello-world binary smoke passes. | P5 |
| v0.70.0-release | macOS AArch64 Dartmouth core fixture suite passes. | P5 |
| v0.71.0 | Cross-platform runtime policy passes output-equivalence tests. | P6 |
| v0.72.0 | Cross-platform output matrix is represented and documented. | P6 |
| v0.73.0 | CLI target selection tests pass for every 1.0 target. | P6 |
| v0.74.0 | Dartmouth version CLI tests pass and BASIC 3 is rejected. | P6 |
| v0.75.0 | Compatibility report golden tests pass. | P6 |
| v0.76.0 | Generated-binary report golden tests pass. | P6 |
| v0.77.0 | User-facing diagnostic golden tests pass. | P6 |
| v0.78.0 | BASIC 1 compatibility sweep passes on implemented targets. | P2 |
| v0.79.0 | BASIC 2 compatibility sweep passes on implemented targets. | P2 |
| v0.80.0-release | BASIC 4 compatibility sweep passes on implemented targets. | P2 |
| v0.81.0 | Cross-version compatibility matrix passes. | P6 |
| v0.82.0 | Cross-target output matrix passes. | P6 |
| v0.83.0 | Parser abuse hardening corpus passes without panics. | P7 |
| v0.84.0 | Relocation and layout abuse tests pass. | P7 |
| v0.85.0 | Runtime fragment abuse tests pass on all 1.0 targets. | P7 |
| v0.86.0 | Dependency, audit, SBOM, modularity, and release gates pass. | P7 |
| v0.87.0 | Reproducible generated-output tests pass. | P7 |
| v0.88.0 | Documentation freeze candidate has complete links and examples. | P7 |
| v0.89.0 | Feature freeze candidate has complete fixture and target matrices. | P7 |
| v0.90.0-release | Performance and size baselines are recorded. | P7 |
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
- `rust-toolchain.toml` pinned to Rust `1.96.1`.
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

### v0.9.0 - Manual Corpus Setup

Goal:

Turn the local Dartmouth manuals into controlled fixture sources without
committing restricted documents.

Deliverables:

- Fixture directory layout.
- Manual provenance notes for versions 1, 2, and 4.
- No committed proprietary/manual PDF content unless license allows it.
- Extracted tiny examples where allowed or handwritten equivalent fixtures.
- Documentation that version 3 is out of scope.

Verification:

- Fixture manifest validates.
- Missing manual paths produce clear local-only warnings, not CI failures.

## Phase 2: Dartmouth BASIC 1 Complete Frontend

### v0.10.0-release - BASIC 1 Line Table

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

Define the source-shaped BASIC 1 representation.

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

### v0.16.0 - BASIC 1 Semantic Validation

Goal:

Validate complete BASIC 1 programs before MIR lowering.

Deliverables:

- BASIC 1 feature matrix.
- Variable resolver.
- Line label resolver.
- Control-flow graph.
- Reachability report.

Verification:

- Missing target diagnostics.
- Unreachable line report tests.
- BASIC 1 feature matrix tests.

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
- Numeric formatting runtime contract if BASIC 1 needs numeric output.
- Runtime error policy.
- Fragment inventory report.

Verification:

- BASIC 1 runtime inventory tests.
- Runtime report golden tests.

### v0.19.0 - BASIC 1 Host Model Runner

Goal:

Prove BASIC 1 semantics without waiting for every native backend.

Deliverables:

- Deterministic host model runner for MIR or semantic model.
- stdout/stdin model.
- exit-code model.
- runtime-error model.

Verification:

- BASIC 1 semantic fixture suite passes in the host model.
- No native executable output is claimed by this tag.

### v0.20.0-release - BASIC 1 Compatibility Sweep

Goal:

Close known BASIC 1 frontend and semantic gaps before adding BASIC 2.

Deliverables:

- BASIC 1 supported-feature matrix.
- BASIC 1 known limitations.
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

### v0.26.0 - BASIC 2 Host Model Runner

Goal:

Prove BASIC 2 semantics before moving to BASIC 4.

Deliverables:

- BASIC 2 host model support.
- BASIC 2 stdin/stdout fixtures.
- BASIC 2 runtime-error fixtures.

Verification:

- BASIC 2 semantic fixture suite passes in the host model.
- BASIC 1 host model suite still passes.

### v0.27.0 - BASIC 2 Compatibility Sweep

Goal:

Close known BASIC 2 gaps before adding BASIC 4.

Deliverables:

- BASIC 2 supported-feature matrix.
- BASIC 2 known limitations.
- BASIC 2 manual-derived fixture suite.
- BASIC 4 syntax rejection suite in BASIC 2 mode.

Verification:

- BASIC 1 fixture suite passes.
- BASIC 2 fixture suite passes.
- Cross-version rejection suite passes.

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

### v0.30.0-release - BASIC 4 Parser Expansion

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

### v0.33.0 - BASIC 4 Host Model Runner

Goal:

Prove BASIC 4 semantics before native backend work becomes the main focus.

Deliverables:

- BASIC 4 host model support.
- BASIC 4 stdin/stdout fixtures.
- BASIC 4 runtime-error fixtures.

Verification:

- BASIC 1, BASIC 2, and BASIC 4 semantic fixture suites pass in the host model.

### v0.34.0 - BASIC 4 Compatibility Sweep

Goal:

Close known BASIC 4 gaps before platform backend work.

Deliverables:

- BASIC 4 supported-feature matrix.
- BASIC 4 known limitations.
- BASIC 4 manual-derived fixture suite.
- Cross-version matrix for BASIC 1, BASIC 2, and BASIC 4.

Verification:

- BASIC 1 fixture suite passes.
- BASIC 2 fixture suite passes.
- BASIC 4 fixture suite passes.
- Cross-version rejection suite passes.

## Phase 5: Runtime Fragments

### v0.35.0 - Runtime Fragment Implementation

Goal:

Implement generated-program helpers without linking an external runtime.

Deliverables:

- Runtime fragment inventory.
- Fragment dependency graph.
- Fragment symbol naming.
- Fragment inclusion report.
- Fragment lowering into target-near LIR.

Verification:

- Fragment selection tests.
- No-unused-fragment tests.

### v0.36.0 - Write And Exit Runtime

Goal:

Provide minimal output and termination behavior.

Deliverables:

- `write_static` fragment contract.
- `exit` fragment contract.
- Platform output/exit ABI abstraction.

Verification:

- Runtime inventory for hello-world programs.
- Platform API report tests.

### v0.37.0 - Integer Formatting Runtime

Goal:

Print numeric values without libc.

Deliverables:

- Signed integer to decimal.
- Newline handling.
- Buffer bounds policy.

Verification:

- Integer formatting unit tests.
- Generated-output golden tests.

### v0.38.0 - Input Runtime

Goal:

Read and parse numeric input without libc.

Deliverables:

- `read_line`.
- `parse_i64` or selected numeric parser.
- Input error exit path.

Verification:

- stdin fixture tests.
- Invalid input tests.

### v0.39.0 - Data And Array Runtime

Goal:

Support DATA streams and arrays in generated binaries.

Deliverables:

- DATA cursor runtime.
- Array bounds runtime.
- Bounds failure exit path.

Verification:

- DATA smoke tests.
- Array bounds tests.

## Phase 6: ELF Writers

### v0.40.0-release - ELF Writer Core

Goal:

Build the common ELF serializer without unsafe header casting.

Deliverables:

- Explicit endian writers.
- ELF identification writer.
- Program header writer.
- Layout validation contracts.

Verification:

- Exact-byte header tests.
- Invalid-layout tests.

### v0.41.0 - ELF64 Tiny Profile

Goal:

Write minimal ELF64 executables for early 64-bit targets.

Deliverables:

- ELF64 header.
- One PT_LOAD tiny profile.
- Entry point validation.
- No dynamic linker.

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

Verification:

- ELF32 segment permission tests.
- Address overflow rejection tests.

## Phase 7: x86 Backends

### v0.45.0 - x86_64 Encoder Core

Goal:

Encode the first x86_64 instruction subset safely.

Deliverables:

- Register model.
- REX prefix handling.
- Immediate moves.
- RIP-relative references.
- Syscall instruction.

Verification:

- Exact-byte instruction tests.
- Relocation placeholder tests.

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

### v0.47.0 - Linux x86_64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable x86_64 Linux ELF64 binary.

Deliverables:

- x86_64 Linux syscall lowering.
- ELF64 integration.
- CLI output path for one program.

Verification:

- Generated binary prints `HELLO`.
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

- Generated fixture suite passes on x86_64 Linux.

### v0.49.0 - x86 32-bit Encoder Core

Goal:

Encode the first 32-bit x86 instruction subset.

Deliverables:

- 32-bit register model.
- Immediate moves.
- Relative branches.
- Linux `int 0x80` or selected syscall ABI policy.

Verification:

- Exact-byte instruction tests.
- ABI documentation tests.

### v0.50.0-release - x86 32-bit Relocations

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

### v0.51.0 - Linux x86 32-bit Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable 32-bit x86 Linux ELF32 binary.

Deliverables:

- x86 Linux syscall lowering.
- ELF32 integration.
- Generated binary report.

Verification:

- Generated binary prints `HELLO` on a Linux environment with 32-bit execution
  support or emulator.

### v0.52.0 - Linux x86 32-bit Dartmouth Core

Goal:

Run core Dartmouth BASIC features on 32-bit x86.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes on x86 32-bit Linux or emulator.

## Phase 8: AArch Backends

### v0.53.0 - AArch64 Encoder Core

Goal:

Encode the first AArch64 instruction subset.

Deliverables:

- Register model.
- Immediate materialization policy.
- PC-relative data reference policy.
- Linux `svc` syscall convention.

Verification:

- Exact-word instruction tests.
- ABI documentation tests.

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

### v0.55.0 - Linux AArch64 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable AArch64 Linux ELF64 binary.

Deliverables:

- AArch64 Linux syscall lowering.
- ELF64 integration.
- Generated binary report.

Verification:

- Generated binary prints `HELLO` on AArch64 Linux or emulator.

### v0.56.0 - Linux AArch64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on AArch64.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes on AArch64 Linux or emulator.

### v0.57.0 - AArch32 Encoder Core

Goal:

Encode the first 32-bit ARM/AArch32 instruction subset.

Deliverables:

- Register model.
- ARM/Thumb mode decision.
- Immediate materialization policy.
- Linux `svc` syscall convention.

Verification:

- Exact-word instruction tests.
- ABI documentation tests.

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

### v0.59.0 - Linux AArch32 Hello World

Goal:

Compile `PRINT "HELLO"`/`END` to a runnable AArch32 Linux ELF32 binary.

Deliverables:

- AArch32 Linux syscall lowering.
- ELF32 integration.
- Generated binary report.

Verification:

- Generated binary prints `HELLO` on AArch32 Linux or emulator.

### v0.60.0-release - Linux AArch32 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on AArch32.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.

Verification:

- Generated fixture suite passes on AArch32 Linux or emulator.

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

Verification:

- Exact-byte PE header tests.
- Invalid-layout tests.

### v0.62.0 - PE64 Tiny Profile

Goal:

Write minimal PE64 executables for Windows x86_64.

Deliverables:

- PE32+ optional header.
- `.text`, `.rdata`, `.data`, and `.bss` section layout.
- Entry point validation.
- Import table policy.
- No external BASIC runtime.

Verification:

- PE64 exact-byte tests.
- PE layout report tests.

### v0.63.0 - Windows x86_64 ABI Lowering

Goal:

Lower Elderheim LIR/runtime calls into the Windows x86_64 ABI.

Deliverables:

- Windows x64 calling convention model.
- Stack alignment and shadow-space policy.
- Process exit strategy.
- Console output strategy.
- Console input strategy.

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

- Generated executable prints `HELLO` on Windows 11 or Windows Server 2025,
  or under an approved Windows CI/emulation path.

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

- Generated fixture suite passes on Windows x86_64.

### v0.66.0 - Mach-O Writer Core

Goal:

Build the common Mach-O serializer needed for macOS output.

Deliverables:

- Mach-O 64-bit header writer.
- Load-command writer.
- Segment/section layout model.
- Entry point command policy.
- Code-signing/ad-hoc signing policy document.

Verification:

- Exact-byte Mach-O header tests.
- Invalid-layout tests.

### v0.67.0 - Mach-O AArch64 Tiny Profile

Goal:

Write minimal Mach-O 64-bit executables for Apple Silicon macOS.

Deliverables:

- `arm64` Mach-O header constants.
- `__TEXT`, `__DATA`, and `__LINKEDIT` layout policy.
- Entry point validation.
- Page alignment policy.

Verification:

- Mach-O layout tests.
- Generated binary report tests.

### v0.68.0 - macOS AArch64 ABI Lowering

Goal:

Lower Elderheim LIR/runtime calls into the macOS Apple Silicon ABI.

Deliverables:

- AArch64 Darwin calling convention model.
- Stack alignment policy.
- Process exit strategy.
- Console output strategy.
- Console input strategy.

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

- Generated executable prints `HELLO` on Apple Silicon macOS or an approved
  macOS CI path.

### v0.70.0-release - macOS AArch64 Dartmouth Core

Goal:

Run core Dartmouth BASIC features on Apple Silicon macOS.

Deliverables:

- Arithmetic.
- Control flow.
- Input.
- Arrays.
- Runtime error exit paths.

Verification:

- Generated fixture suite passes on Apple Silicon macOS.

### v0.71.0 - Cross-Platform Runtime Policy

Goal:

Unify runtime-fragment behavior across Linux, Windows, and macOS.

Deliverables:

- Platform syscall/API inventory.
- Exit-code policy.
- Newline policy.
- Console encoding policy.
- Runtime error message policy.

Verification:

- Runtime report tests for all 1.0 platforms.
- Output-equivalence tests for portable programs.

### v0.72.0 - Cross-Platform Output Matrix

Goal:

Prove that all 1.0 output formats are represented before CLI/report work.

Deliverables:

- Linux ELF32/ELF64 matrix.
- Windows PE64 matrix.
- macOS Mach-O aarch64 matrix.
- CI/emulation requirements document.
- Known platform limitation list.

Verification:

- Target matrix tests.
- Format matrix tests.
- Documentation link checks.

## Phase 10: CLI, Reports, And User Experience

### v0.73.0 - CLI Target Selection

Goal:

Make target selection stable and testable.

Deliverables:

- `--target linux-x86`.
- `--target linux-x86_64`.
- `--target linux-aarch32`.
- `--target linux-aarch64`.
- `--target windows-x86_64`.
- `--target macos-aarch64`.
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
- Known limitations.

Verification:

- Version 1 fixtures pass on all implemented targets.

### v0.79.0 - Dartmouth BASIC 2 Compatibility Sweep

Goal:

Close known version 2 compatibility gaps.

Deliverables:

- Version 2 feature matrix.
- Version 2 fixture suite.
- Known limitations.

Verification:

- Version 2 fixtures pass on all implemented targets.

### v0.80.0-release - Dartmouth BASIC 4 Compatibility Sweep

Goal:

Close known version 4 compatibility gaps.

Deliverables:

- Version 4 feature matrix.
- Version 4 fixture suite.
- Known limitations.

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

- Fixture runner for Linux x86, Linux x86_64, Linux AArch32, Linux AArch64,
  Windows x86_64, and macOS AArch64.
- Output comparison.
- Exit-code comparison.
- Runtime-fragment comparison.

Verification:

- Cross-target fixture suite passes.

## Phase 12: Security Hardening

### v0.83.0 - Parser Abuse Hardening

Goal:

Harden source ingestion and parsing.

Deliverables:

- Malformed source corpus.
- Deep expression limits.
- Huge line-number tests.
- Diagnostic flood limits.

Verification:

- Abuse tests pass.
- No panics.

### v0.84.0 - Relocation And Layout Hardening

Goal:

Harden machine-code and executable layout math.

Deliverables:

- Relocation fuzz-style unit tests.
- Address overflow tests.
- Segment overlap tests.
- Entry-point validation tests.

Verification:

- Layout abuse tests pass for ELF32 and ELF64.

### v0.85.0 - Runtime Fragment Hardening

Goal:

Harden generated-program runtime helpers.

Deliverables:

- Input bounds tests.
- Array bounds tests.
- Data cursor bounds tests.
- Division failure tests.

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
- Known limitations.

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

### v0.90.0-release - Performance And Size Baseline

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

- Follow-up pentest review passes or explicitly defers non-blocking findings.

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
- No external backend, assembler, linker, libc, or BASIC runtime dependency for
  generated programs.
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
- Pentest report status `PASS`.

Post-1.0 directions:

- Other documented BASIC variants.
- Windows 32-bit output, if it becomes useful enough to justify the work.
- Intel macOS output, only if there is a clear support requirement.
- BSD and Android target hardening.
- Other older language-family crates only after source material and scope are
  ready.
- Aesynx output path when Aesynx is ready.
