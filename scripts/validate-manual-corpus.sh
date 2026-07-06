#!/usr/bin/env sh
set -eu

manual_root="${ELDERHEIM_MANUAL_ROOT:-/home/eldryoth/Work/test/basicmanuals}"

test -f docs/manual-corpus.md
test -f docs/languages/dartmouth-basic-1.md
test -f examples/dartmouth-basic-1/manifest.txt

while IFS= read -r example; do
    [ -n "$example" ] || continue
    test -f "examples/dartmouth-basic-1/$example"
done < examples/dartmouth-basic-1/manifest.txt

for manual in \
    "first edition may 1964.pdf" \
    "second edition october 1964.pdf" \
    "196801_BASIC_4th_Edition_text.pdf"
do
    if [ -f "$manual_root/$manual" ]; then
        echo "manual found: $manual_root/$manual"
    else
        echo "warning: local manual not found: $manual_root/$manual" >&2
    fi
done
