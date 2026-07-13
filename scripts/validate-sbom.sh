#!/usr/bin/env sh
set -eu

sbom="${1:-sbom/elderheim.spdx.json}"

if ! command -v jq >/dev/null 2>&1; then
    echo "jq is required to validate SPDX SBOM integrity" >&2
    exit 1
fi

test -s "$sbom"

jq -e '
  .spdxVersion == "SPDX-2.3"
  and .dataLicense == "CC0-1.0"
  and (.SPDXID == "SPDXRef-DOCUMENT")
' "$sbom" >/dev/null

jq -e '
  [.. | objects | .SPDXID? // empty] as $ids
  | ($ids | length) == ($ids | unique | length)
' "$sbom" >/dev/null

jq -e '
  ([.. | objects | .SPDXID? // empty] | unique) as $ids
  | all(
      .relationships[]?;
      (.spdxElementId as $source | ($ids | index($source)) != null)
      and
      (
        .relatedSpdxElement as $target
        | $target == "NONE"
          or $target == "NOASSERTION"
          or ($ids | index($target)) != null
      )
    )
' "$sbom" >/dev/null
