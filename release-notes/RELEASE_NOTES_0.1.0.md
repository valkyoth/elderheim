# Elderheim 0.1.0 Release Notes

Status: release-candidate scaffold

## Scope

`0.1.0` initializes the Elderheim repository and compiler workspace.

## Added

- Rust workspace pinned to stable `1.96.1`.
- EUPL-1.2 licensing.
- No third-party crate dependencies.
- no_std crate skeletons for core, IR, target, x86 backend, ELF writer,
  and Dartmouth BASIC.
- Initial target identifiers for Linux `x86`, `x86_64`, `aarch32`, and
  `aarch64`.
- Initial target identifiers for Windows `x86_64` and macOS Apple Silicon
  `aarch64`.
- Thin `elderheim` CLI shell.
- Implementation plan and release plan.
- Security, modularity, unsafe, toolchain, and supply-chain policy docs.
- Local check scripts.

## Known Limitations

- No parser is implemented yet.
- No executable writer is implemented yet.
- Local verification requires Rust `1.96.1`.
