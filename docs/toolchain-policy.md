# Elderheim Toolchain Policy

Elderheim pins stable Rust `1.96.1` in `rust-toolchain.toml`.

Rules:

- Update the pinned toolchain only after checking the current latest stable
  Rust release.
- Keep `workspace.package.rust-version` aligned with the pinned stable version
  until an explicit MSRV policy is introduced.
- Do not require nightly for normal builds.
- Keep crate dependencies at zero until a reviewed need is documented.
- If a dependency becomes necessary, prefer a small no_std-compatible crate and
  document why internal implementation is worse.

The local toolchain in this environment is currently `1.96.1`, matching the
pinned stable release.
