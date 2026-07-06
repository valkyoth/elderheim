#!/usr/bin/env sh
set -eu

tag="${1:-}"
case "$tag" in
    v0.10.0 | v0.20.0 | v0.30.0 | v0.40.0 | v0.50.0 | v0.60.0 | v0.70.0 | v0.80.0 | v0.90.0)
        echo "immutable release tag already exists: ${tag}; use ${tag}-release" >&2
        exit 2
        ;;
    v[0-9]*.[0-9]*.[0-9]* | v[0-9]*.[0-9]*.[0-9]*-release) ;;
    *)
        echo "usage: scripts/validate-release-readiness.sh vX.Y.Z" >&2
        echo "       locked milestones use vX.Y.Z-release" >&2
        exit 2
        ;;
esac

version="${tag#v}"
release_notes="release-notes/RELEASE_NOTES_${version}.md"
pentest_report="security/pentest/${version}.md"

if git rev-parse -q --verify "refs/tags/${tag}" >/dev/null; then
    echo "tag already exists locally: ${tag}" >&2
    exit 1
fi

if [ -f PENTEST.md ]; then
    echo "root PENTEST.md is temporary scratch input and must be removed" >&2
    exit 1
fi

if [ ! -f "$release_notes" ]; then
    echo "missing release notes: ${release_notes}" >&2
    exit 1
fi

if [ ! -f "$pentest_report" ]; then
    echo "missing pentest report: ${pentest_report}" >&2
    exit 1
fi

grep -Eq '^Status: (PASS|DRAFT)$' "$pentest_report"
grep -Eq '^Commit: (TBD|[0-9a-f]{40})$' "$pentest_report"
grep -Eq '^Tester: .+' "$pentest_report"
grep -Eq '^CodeQL: (TBD|PASS|FINDINGS)$' "$pentest_report"
grep -Eq '^Scope:' "$pentest_report"
