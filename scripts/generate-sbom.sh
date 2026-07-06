#!/usr/bin/env sh
set -eu

mkdir -p sbom
cargo sbom --output-format spdx_json_2_3 > sbom/elderheim.spdx.json
test -s sbom/elderheim.spdx.json
