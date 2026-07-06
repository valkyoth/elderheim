#!/usr/bin/env sh
set -eu

mode="${1:-check}"

if [ "$mode" != "check" ]; then
    echo "usage: scripts/validate-modularity-policy.sh check" >&2
    exit 2
fi

status=0

find crates tools -name '*.rs' -type f | while IFS= read -r file; do
    lines="$(wc -l < "$file")"
    if [ "$lines" -gt 500 ]; then
        echo "file exceeds 500 lines: $file ($lines)" >&2
        status=1
    fi
done

exit "$status"
