#!/usr/bin/env sh
set -eu

mkdir -p sbom
raw="$(mktemp sbom/.elderheim.raw.XXXXXX)"
normalized="$(mktemp sbom/.elderheim.validated.XXXXXX)"
trap 'rm -f "$raw" "$normalized"' EXIT HUP INT TERM

cargo sbom --output-format spdx_json_2_3 > "$raw"
test -s "$raw"

# cargo-sbom 0.10.0 duplicates identical workspace-root Cargo.lock entries.
# Refuse conflicting duplicates, then collapse only byte-identical file records.
jq -e '
  [
    (.files // [])
    | group_by(.SPDXID)[]
    | select(length > 1)
    | (unique | length == 1)
  ]
  | all
' "$raw" >/dev/null

jq '
  .files |= unique_by(.SPDXID)
  | .creationInfo.creators += ["Tool: elderheim-sbom-normalizer-v0.13.0"]
' "$raw" > "$normalized"

scripts/validate-sbom.sh "$normalized"
chmod 0644 "$normalized"
mv "$normalized" sbom/elderheim.spdx.json
