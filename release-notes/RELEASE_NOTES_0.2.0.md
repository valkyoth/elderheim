# Elderheim 0.2.0 Release Notes

Status: release-candidate scaffold

## Scope

`0.2.0` completes the target-matrix skeleton stop. It does not add parsing,
lowering, code generation, executable writing, or Dartmouth BASIC semantics.

## Added

- Canonical CLI-visible names for every supported 1.0 target:
  - `linux-x86-elf32`
  - `linux-x86_64-elf64`
  - `linux-aarch32-elf32`
  - `linux-aarch64-elf64`
  - `windows-x86_64-pe64`
  - `macos-aarch64-macho64`
- no_std target-name parsing in `elderheim-target`.
- Stable target rejection diagnostics:
  - `E-TARGET-EMPTY`
  - `E-TARGET-SHAPE`
  - `E-TARGET-OS`
  - `E-TARGET-ARCH`
  - `E-TARGET-FORMAT`
  - `E-TARGET-UNSUPPORTED`
- CLI target listing with `elderheim --list-targets`.
- CLI target validation with `elderheim --target <os-architecture-format>`.
- Unit tests proving the 1.0 target matrix and unsupported target diagnostics.

## Scope Exclusions

- Parser implementation is not part of the `0.2.0` target-matrix tag.
- Executable-writer implementation is not part of the `0.2.0` target-matrix
  tag.
- Target names are validation contracts only; they do not imply executable
  output support yet.
