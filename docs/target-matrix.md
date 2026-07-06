# Elderheim Target Matrix

Status: active 0.2.0 contract

Elderheim exposes supported target names as explicit
`os-architecture-format` strings. These names are validation contracts for the
1.0 target matrix. They do not imply executable output support before the
backend and executable-writer milestones are complete.

## Supported 1.0 Targets

| CLI target | Operating system | Architecture | Executable format |
| --- | --- | --- | --- |
| `linux-x86-elf32` | Linux | `x86` | ELF32 |
| `linux-x86_64-elf64` | Linux | `x86_64` | ELF64 |
| `linux-aarch32-elf32` | Linux | `aarch32` | ELF32 |
| `linux-aarch64-elf64` | Linux | `aarch64` | ELF64 |
| `windows-x86_64-pe64` | Windows | `x86_64` | PE64 |
| `macos-aarch64-macho64` | macOS Apple Silicon | `aarch64` | Mach-O 64 |

## Rejection Diagnostics

Unsupported target strings fail closed with stable diagnostics:

| Code | Meaning |
| --- | --- |
| `E-TARGET-EMPTY` | Target name is empty. |
| `E-TARGET-SHAPE` | Target name does not use `os-architecture-format`. |
| `E-TARGET-OS` | Operating system component is not recognized. |
| `E-TARGET-ARCH` | Architecture component is not recognized. |
| `E-TARGET-FORMAT` | Executable format component is not recognized. |
| `E-TARGET-UNSUPPORTED` | Components are recognized, but the combination is not part of the 1.0 matrix. |

## CLI Checks

List the supported matrix:

```bash
cargo run -p elderheim --bin elderheim -- --list-targets
```

Validate a target:

```bash
cargo run -p elderheim --bin elderheim -- --target linux-x86_64-elf64
```
