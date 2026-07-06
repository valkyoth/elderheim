# Contributing To Elderheim

Elderheim is a security-sensitive compiler project. Contributions should keep
the compiler small, explicit, documented, and testable.

## License

Elderheim is licensed under the European Union Public Licence 1.2. By
contributing, you agree that your contribution is provided under the same
license.

## Development Setup

Use the pinned Rust toolchain from `rust-toolchain.toml`.

```bash
scripts/checks.sh
```

## Security-Sensitive Areas

Treat these areas as high risk:

- source parsing and dialect normalization;
- integer, offset, and relocation arithmetic;
- executable-format writing;
- generated runtime fragments;
- CLI file paths and output permissions;
- dependency or CI workflow changes.

Do not post exploitable security details in public issues. Follow
[SECURITY.md](../SECURITY.md).

## Dependency Policy

Elderheim starts with no third-party crate dependencies.

When a dependency is proposed:

- document why local implementation is worse;
- prefer no_std-compatible crates;
- use crates.io releases unless there is a strong reason not to;
- avoid git dependencies;
- check maintenance status and license;
- run `cargo deny check` and `cargo audit` when those tools are installed.

## Pull Requests

Good pull requests are small enough to review and include:

- a clear summary;
- tests for behavior changes;
- docs for user-facing or policy changes;
- security notes for risky areas.
