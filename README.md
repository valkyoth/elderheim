<p align="center">
  <b>Rust-native compiler platform for legacy languages, starting with Dartmouth BASIC.</b><br>
  Multi-frontend by design. Direct machine-code and executable-format output. Built for standalone release binaries.
</p>

<div align="center">
  <a href="docs/IMPLEMENTATION_PLAN.md">Implementation Plan</a>
  ·
  <a href="docs/RELEASE_PLAN.md">Release Plan</a>
  ·
  <a href="docs/RELEASE_PLAN.md#stop-gates-and-pentest-classes">Tag Stops</a>
  ·
  <a href="docs/modularity-policy.md">Modularity</a>
  ·
  <a href="docs/security-controls.md">Security Controls</a>
  ·
  <a href="docs/threat-model.md">Threat Model</a>
  ·
  <a href="docs/supply-chain-security.md">Supply Chain</a>
  ·
  <a href="SECURITY.md">Security</a>
</div>

<br>

<p align="center">
  <img src="./.github/images/elderheim.webp" alt="elderheim overview">
</p>

# elderheim

elderheim is a universal compiler platform for legacy languages. The first
supported language family is Dartmouth BASIC, specifically the locally planned
manual-backed profiles for Dartmouth BASIC versions 1, 2, and 4.

The intended release model is one downloadable `elderheim` compiler binary per
supported operating system and architecture. A user downloads `elderheim`,
chooses the source dialect explicitly, and compiles old source files without
installing Rust, `rustc`, Cargo, a C compiler, or a platform linker for the
supported release path.

The final supported release path is meant to be self-contained:

- A user downloads one `elderheim` binary for their OS and architecture.
- A user runs `elderheim --dartmouth-basic-1 program.bas -o program`.
- No Rust install is required.
- No `rustc` is required.
- No Cargo is required.
- No C compiler is required.
- No external assembler is required.
- No external linker is required for the supported release path.
- No Cranelift or LLVM backend is used.
- Old source code compiles through `elderheim` itself.

The first stable goal is `1.0.0`: prove the compiler platform with complete
Dartmouth BASIC 1, then complete Dartmouth BASIC 2, then complete Dartmouth
BASIC 4. Dartmouth BASIC 3 stays reserved until official documentation is
available. Other BASIC variants and non-BASIC language families are future
frontends under `crates/languages/`, not implementation claims in the
foundation release.

elderheim is licensed under `EUPL-1.2`.

## Historical Documentation

Elderheim depends on primary historical manuals to keep dialect support
accurate. The first local reference set is:

- Dartmouth BASIC First Edition, May 1964.
- Dartmouth BASIC Second Edition, October 1964.
- Dartmouth BASIC Fourth Edition, January 1968 text export.

If primary documentation for Dartmouth BASIC Third Edition or later Dartmouth
editions becomes available, those profiles can be planned as explicit future
compatibility releases. They are not part of the `1.0.0` scope.

## What Works Today

`0.1.0` is the active foundation. It initializes the workspace, security
policy, release planning, target identifiers, crate boundaries, and local
verification gates.

No Dartmouth BASIC parser or executable writer is implemented yet. The roadmap
intentionally starts with compiler substrate, then makes BASIC 1 complete,
then BASIC 2 complete, then BASIC 4 complete.

### Compiler Foundation

| Capability | Status | Notes |
| --- | --- | --- |
| Cargo workspace | Working | Shared compiler crates live directly under `crates/`; language frontends live under `crates/languages/`. |
| Rust toolchain pin | Working | Stable Rust `1.96.1`, edition 2024, workspace resolver `3`. |
| License | Working | EUPL-1.2. |
| no_std library skeletons | Working | Core/facade crates are prepared for no_std production logic. |
| Target matrix identifiers | Working | Linux `x86`/`x86_64`/`aarch32`/`aarch64`, Windows `x86_64`, and macOS Apple Silicon `aarch64` are represented. |
| Dartmouth BASIC crate | Scaffolded | Active first language-family crate: `crates/languages/elderheim-dartmouth-basic`. |
| Direct backend plan | Planned | Native output is planned through Elderheim-owned instruction encoders and executable writers, not Cranelift, LLVM, C, or Rust transpilation. |
| ELF writer | Scaffolded | `elderheim-format-elf` is present for future ELF32/ELF64 work. |
| x86 backend | Scaffolded | `elderheim-backend-x86` is present for future x86 32-bit and x86_64 work. |
| Release/security gates | Working | Formatting, doc links, modularity, clippy, tests, cargo-deny, cargo-audit, and SBOM generation pass locally. |
| Pentest/tag stops | Planned | Every planned tag has a stop gate and pentest class in the release plan. |

### Language Support

Compatibility is tracked per concrete language or dialect, not by loose family
names. A dialect is marked complete only after manual-backed fixtures, target
fixtures, release gates, and pentest evidence pass.

| Language or dialect | Status | Comment |
| --- | --- | --- |
| Dartmouth BASIC First Edition (`dartmouth-basic-1`) | Planned for first implementation line | BASIC 1 must reach a complete supported subset before BASIC 2 begins. |
| Dartmouth BASIC Second Edition (`dartmouth-basic-2`) | Planned after BASIC 1 | Added as an explicit compatibility expansion over proven BASIC 1 behavior. |
| Dartmouth BASIC Third Edition (`dartmouth-basic-3`) | Reserved | No official documentation is available locally; not part of `1.0.0`. |
| Dartmouth BASIC Fourth Edition (`dartmouth-basic-4`) | Planned after BASIC 2 | Added only after BASIC 2 reaches its compatibility stop. |
| Other Dartmouth BASIC editions | Reserved | Need primary manuals before scheduling. |
| Other BASIC variants | Future | Planned only after source material and release scope are ready. |
| Non-BASIC languages | Future | The platform is designed for future language-family crates, but none are active in the foundation. |

## Why elderheim

- **Standalone compiler goal**: released binaries should compile supported
  source files without requiring users to install Rust, Cargo, a C compiler, an
  external assembler, or an external linker for the supported release path.
- **Rust first**: memory-safe implementation with a pinned modern Rust
  toolchain.
- **Direct native output**: Elderheim owns the machine-code backend and
  executable-format writer instead of shelling out to Cranelift, LLVM, C,
  Rust, assemblers, or linkers.
- **Language-family frontend crates**: each language family gets its own crate
  under `crates/languages/` when source material and release scope justify it.
- **Security first**: unsupported constructs fail explicitly, dependencies are
  audited, releases require SBOM evidence, and every tag has a pentest class.

## Quick Start

Run the full local gate:

```bash
scripts/checks.sh
```

Run the dependency and advisory gates:

```bash
cargo deny check
cargo audit
```

Generate an SBOM:

```bash
scripts/generate-sbom.sh
```

Run the current CLI scaffold:

```bash
cargo run -p elderheim --bin elderheim
```

## Workspace

```text
elderheim/
├── crates/
│   ├── elderheim/                  # no_std facade library and CLI shell
│   ├── elderheim-core/             # spans, diagnostics, limits, IDs
│   ├── elderheim-ir/               # HIR/MIR/LIR contracts
│   ├── elderheim-target/           # target and format identifiers
│   ├── elderheim-backend-x86/      # x86 32-bit and x86_64 backend contracts
│   ├── elderheim-format-elf/       # ELF32/ELF64 writer contracts
│   └── languages/
│       └── elderheim-dartmouth-basic/
├── docs/
├── release-notes/
├── scripts/
├── security/
└── tools/
```

## Documentation

| Document | Purpose |
| --- | --- |
| [Implementation Plan](docs/IMPLEMENTATION_PLAN.md) | Compiler architecture, workspace shape, Dartmouth sequencing, and output strategy. |
| [Release Plan](docs/RELEASE_PLAN.md) | Granular roadmap from `0.1.0` through `1.0.0`. |
| [Tag Stops](docs/RELEASE_PLAN.md#stop-gates-and-pentest-classes) | Stop gate and pentest class for every planned tag. |
| [Modularity Policy](docs/modularity-policy.md) | Crate split rules and 500-line source-file policy. |
| [Unsafe Policy](docs/unsafe-policy.md) | Unsafe admission rules and serialization safety policy. |
| [Toolchain Policy](docs/toolchain-policy.md) | Rust version pin and tooling expectations. |
| [Security Controls](docs/security-controls.md) | Required compiler, release, and CodeQL controls. |
| [Threat Model](docs/threat-model.md) | Assets, trust boundaries, and residual risks. |
| [Supply-Chain Security](docs/supply-chain-security.md) | Dependency and tooling review policy. |
| [Security Policy](SECURITY.md) | Security checks and reporting guidance. |

## Release Direction

The project does not aim to make one giant parser that guesses every old
language. Users should choose the dialect explicitly:

```bash
elderheim --dartmouth-basic-1 program.bas -o program
elderheim --dartmouth-basic-2 program.bas -o program
elderheim --dartmouth-basic-4 program.bas -o program
```

Future language families should live in their own local workspace crates under
`crates/languages/`. Shared compiler infrastructure stays in the core crates,
while source-language rules remain isolated inside each language-family
frontend.
