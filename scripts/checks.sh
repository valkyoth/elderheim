#!/usr/bin/env sh
set -eu

cargo fmt --all --check
scripts/check_shell_syntax.sh
scripts/check_doc_links.sh
scripts/validate-modularity-policy.sh check
scripts/validate-manual-corpus.sh
scripts/validate-sbom.sh
scripts/test-sbom-validation.sh
scripts/validate-release-metadata.sh
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
cargo audit
