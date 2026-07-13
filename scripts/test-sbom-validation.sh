#!/usr/bin/env sh
set -eu

fixture="$(mktemp sbom/.elderheim.invalid.XXXXXX)"
trap 'rm -f "$fixture"' EXIT HUP INT TERM

jq '.files += [.files[0]]' sbom/elderheim.spdx.json > "$fixture"

if scripts/validate-sbom.sh "$fixture" >/dev/null 2>&1; then
    echo "SBOM validator accepted a duplicate SPDX identifier" >&2
    exit 1
fi
