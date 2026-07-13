#!/usr/bin/env sh
set -eu

tag="${1:-}"
case "$tag" in
    v0.10.0 | v0.20.0 | v0.30.0 | v0.40.0 | v0.50.0 | v0.60.0 | v0.70.0 | v0.80.0 | v0.90.0)
        echo "immutable release tag already exists: ${tag}; use ${tag}-elderheim" >&2
        exit 2
        ;;
    v0.10.0-elderheim | v0.20.0-elderheim | v0.30.0-elderheim | v0.40.0-elderheim | v0.50.0-elderheim | v0.60.0-elderheim | v0.70.0-elderheim | v0.80.0-elderheim | v0.90.0-elderheim)
        version="${tag#v}"
        version="${version%-elderheim}"
        ;;
    v*) version="${tag#v}" ;;
    *)
        echo "usage: scripts/validate-release-candidate.sh vX.Y.Z" >&2
        echo "       locked milestones use vX.Y.Z-elderheim" >&2
        exit 2
        ;;
esac

printf '%s' "$version" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+$' || {
    echo "invalid version segment: ${version}" >&2
    exit 2
}

release_notes="release-notes/RELEASE_NOTES_${version}.md"
pentest_report="security/pentest/${version}.md"

scripts/validate-sbom.sh

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

grep -Eq '^Status: (DRAFT|PASS)$' "$pentest_report" || {
    echo "pentest report status is invalid: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Reviewed-Commit: (TBD|[0-9a-f]{40})$' "$pentest_report" || {
    echo "pentest report reviewed commit field is invalid: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Remediation-Commit: (TBD|[0-9a-f]{40})$' "$pentest_report" || {
    echo "pentest report remediation commit field is invalid: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Tester: .+' "$pentest_report" || {
    echo "pentest report is missing tester identity: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Scope:' "$pentest_report" || {
    echo "pentest report is missing scope: ${pentest_report}" >&2
    exit 1
}
