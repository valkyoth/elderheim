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
        echo "usage: scripts/validate-release-readiness.sh vX.Y.Z" >&2
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

grep -Eq '^Status: PASS$' "$pentest_report" || {
    echo "pentest report is not PASS: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Reviewed-Commit: [0-9a-f]{40}$' "$pentest_report" || {
    echo "pentest report does not name the reviewed commit: ${pentest_report}" >&2
    exit 1
}

grep -Eq '^Remediation-Commit: [0-9a-f]{40}$' "$pentest_report" || {
    echo "pentest report does not name the remediation commit: ${pentest_report}" >&2
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

branch="$(git branch --show-current)"
if [ -z "$branch" ]; then
    echo "release readiness must run on a named branch before tagging" >&2
    exit 1
fi

local_head="$(git rev-parse HEAD)"
if ! remote_line="$(git ls-remote --heads origin -- "$branch")"; then
    echo "could not read origin/${branch}" >&2
    exit 1
fi

if [ -z "$remote_line" ]; then
    echo "origin/${branch} does not exist" >&2
    exit 1
fi

remote_head="${remote_line%%	*}"
if [ "$local_head" != "$remote_head" ]; then
    echo "release commit is not pushed to origin/${branch}: ${local_head}" >&2
    exit 1
fi
