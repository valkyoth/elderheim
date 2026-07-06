# Security Policy

Elderheim is security-sensitive compiler infrastructure. Treat parser,
normalization, relocation, executable-format, runtime-fragment, dependency, and
CI changes as high-risk until tested.

## Routine Checks

Run these regularly and before release candidates:

```bash
scripts/checks.sh
cargo deny check
cargo audit
scripts/generate-sbom.sh
```

GitHub Actions run normal Rust CI. GitHub CodeQL default setup should be enabled
in the repository security settings. Keep only one active CodeQL configuration:
do not add an advanced CodeQL workflow while default setup is active.

After the release-candidate commit is pushed, GitHub CodeQL default setup runs
on that commit. If CodeQL reports findings, fix them, update the matching
`security/pentest/<version>.md` report, and make a new release-candidate commit.

## Dependency Policy

The dependency policy lives in `deny.toml`. Unknown registries and git sources
are denied by default. License exceptions must be narrow and documented.

Elderheim starts with no third-party crate dependencies. Any dependency change
is a supply-chain change and needs review.

## Generated Binary Policy

Generated programs must not depend on rustc, Cargo, LLVM, Cranelift, system
assemblers, system linkers, libc, or external BASIC runtimes.

Executable writers must use explicit byte serialization and checked arithmetic.
Do not serialize executable headers by casting Rust structs to bytes.

## Reporting

Do not publish exploitable security details before a fix is available. Open a
private security advisory or contact the maintainers directly once the public
repository security channel is configured.
