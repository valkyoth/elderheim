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
- Valid artifacts from different source, semantic, option, limit, IR-schema, or
  target-service contracts being mixed into one apparently valid compilation.
- A faulty shared canonicalizer making production and verification agree on the
  same ambiguous encoding or digest domain.
- Parallel hash implementations or caller-selected domain tags causing source,
  semantic, target, image, and certificate digests to drift or cross domains.
- A correctly hashed but unsupported semantic, compile-configuration, or target
  contract being mistaken for a supported validation capability.
- Externally supplied reports, certificates, fingerprints, resource plans, or
  executable bytes being trusted as authentic or used to reconstruct an
  in-memory validation capability.
- Cache or report logic treating non-cryptographic source identifiers as proof
  that two source byte streams are identical.
- Source-controlled IR identifiers causing algorithmic-complexity denial of
  service during validation.
- File offsets being confused with runtime virtual addresses in executable
  layout checks.
- Non-UTF-8 operating-system arguments causing a CLI panic.
- Ambiguous SBOM identifiers corrupting release evidence.
- Unicode formatting controls spoofing human-readable compiler snapshots.
- Literal escape-looking text colliding with generated snapshot escapes.

## Controls

- `no_std` core crates.
- No third-party dependencies in the foundation.
- Explicit target and format layers.
- Explicit Dartmouth BASIC version profiles.
- Opaque content-bound semantic and compilation identities preserved through
  every artifact boundary and rejected on mismatch.
- Identity constructors consume sealed supported-contract and validated
  compile-configuration capabilities; hashing never replaces validation.
- Approved semantics-preserving transformations retain identity only after
  transactional construction and complete revalidation from one predecessor.
- Independently derived canonical-encoding known-answer vectors plus strict
  decoders that reject ambiguity, duplication, trailing data, malformed
  lengths, noncanonical versions, and digest-domain substitution.
- One shared typed digest primitive with sealed domains and non-interchangeable
  outputs; independent validators share only the primitive and independently
  assemble/parse canonical preimages.
- `SourceId` is not a security, cache-integrity, or source-equality boundary.
  Diagnostic and cursor source-id checks are best-effort misuse detection
  inside one trusted compilation session, not adversarial equality proofs.
- Local checks for formatting, linting, tests, docs, and file-size policy.
- Bounded subquadratic IR validation and checked executable-layout arithmetic.
- Printable-ASCII-only snapshot rendering with escaped non-ASCII code points.
- Canonical snapshot escaping that doubles source backslashes.
- SPDX identifier and relationship validation before SBOM publication.
- Release gates that require security review before tags.

## Residual Trust Boundary

Serialized reports, certificates, fingerprints, resource plans, and executable
bytes are untrusted data, even when their internal hashes agree. They cannot
directly reconstruct `VerifiedImage`, `CompilationIdentity`, or any other
validation capability. Re-verification independently parses the executable,
validates the resource plan and compile configuration, recomputes all digests
and fingerprints, and matches sealed supported semantic and target-service
contracts.

These identities and certificates establish deterministic identity and
internal consistency. They do not authenticate a producer or signer. External
artifact authenticity, if added later, requires a separate explicit roadmap
decision and key/trust policy; format-required loader signature metadata does
not imply that authenticity claim.
