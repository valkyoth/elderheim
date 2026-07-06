# Elderheim Threat Model

Elderheim treats compiler output as security-sensitive.

## Assets

- Source program meaning.
- Generated executable integrity.
- Deterministic compiler behavior.
- Dialect compatibility reports.
- User file system paths passed to the CLI.

## Early Threats

- Malformed historic source causing panics or unchecked arithmetic.
- Incorrect relocation math causing generated control-flow corruption.
- Incorrect executable layout producing RWX or invalid segments.
- Any generated-program dependency on host compilers, linkers, libc, or
  runtimes.
- Dialect confusion where one Dartmouth BASIC version silently accepts another
  version's semantics.

## Controls

- `no_std` core crates.
- No third-party dependencies in the foundation.
- Explicit target and format layers.
- Explicit Dartmouth BASIC version profiles.
- Local checks for formatting, linting, tests, docs, and file-size policy.
- Release gates that require security review before tags.
