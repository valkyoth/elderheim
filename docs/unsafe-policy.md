# Elderheim Unsafe Policy

Elderheim production crates start with `unsafe_code = "forbid"`.

Unsafe Rust is not expected in `v0.1.0`. Future unsafe code may be admitted
only for narrow implementation zones such as generated executable validation,
platform startup experiments, or carefully reviewed byte-level primitives.

Admission rules:

- Prefer safe Rust and explicit byte serialization.
- Do not cast ELF, PE, or Mach-O structs into byte slices.
- Every unsafe block must have a `SAFETY:` comment covering validity,
  alignment, aliasing, lifetime, and concurrency assumptions.
- Every crate with admitted unsafe code must document that boundary.
- Unsafe code must have direct tests around the invariant it relies on.

Generated programs may contain raw machine code, but the Rust code that emits
that machine code should remain safe wherever possible.
