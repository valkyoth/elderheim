#!/usr/bin/env sh
set -eu

test ! -f PENTEST.md
test -f LICENSE
test -f SECURITY.md
test -f CHANGELOG.md
test -f README.md
test -f Cargo.toml
test -f rust-toolchain.toml
test -f deny.toml
test -x scripts/checks.sh
test -x scripts/validate-modularity-policy.sh
test -x scripts/validate-release-candidate.sh
test -x scripts/validate-release-readiness.sh
test -x scripts/generate-sbom.sh
test -f docs/IMPLEMENTATION_PLAN.md
test -f docs/RELEASE_PLAN.md
test -f docs/release-procedure.md
test -f docs/modularity-policy.md
test -f docs/unsafe-policy.md
test -f docs/threat-model.md
test -f docs/security-controls.md
test -f docs/supply-chain-security.md
test -f docs/toolchain-policy.md
test -f release-notes/RELEASE_NOTES_0.1.0.md
test -f security/pentest/0.1.0.md
grep -Eq '^Reviewed-Commit: (TBD|[0-9a-f]{40})$' security/pentest/0.1.0.md
grep -Eq '^Remediation-Commit: (TBD|[0-9a-f]{40})$' security/pentest/0.1.0.md

grep -q 'license = "EUPL-1.2"' Cargo.toml
grep -q 'repository = "https://github.com/valkyoth/elderheim"' Cargo.toml
grep -q 'rust-version = "1.96.1"' Cargo.toml
grep -q 'channel = "1.96.1"' rust-toolchain.toml
