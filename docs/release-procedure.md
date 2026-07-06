# Elderheim Release Procedure

Status: active policy

## Commit Policy

Local commits may be made regularly while work is progressing. Pushes are done
by the maintainer, not by the automation agent.

Every commit should keep the repository in a reviewable state:

- scoped change set;
- no secrets, local manuals, or scratch pentest inputs;
- tests or documentation updated for behavior changes;
- `scripts/checks.sh` passing before release-related commits.

## Tag Policy

Tags are release events and must follow the pentest procedure. Do not tag only
because the code compiles.

Before a tag:

- release notes must exist under `release-notes/`;
- the final pentest report must exist under `security/pentest/`;
- the report must name the reviewed commit and remediation commit;
- the remediation commit must already contain the `Status: PASS` report and be
  the exact commit that will be tagged;
- the remediation commit must already be pushed and green in GitHub checks;
- any release-blocking finding must be fixed before `Status: PASS`;
- any non-blocking finding must have an explicit follow-up release;
- root `PENTEST.md` must be removed after its contents are incorporated.

## Pentest Input Flow

`PENTEST.md` is temporary scratch input supplied during release review. It must
not be committed and must not exist when release readiness is checked.

The normal loop is:

1. The maintainer supplies root `PENTEST.md`.
2. The report is incorporated into `security/pentest/<version>.md`.
3. Root `PENTEST.md` is removed.
4. Blocking findings are fixed and committed locally.
5. The maintainer retests and either supplies a new `PENTEST.md` or asks for a
   full release-candidate check.
6. A release-candidate commit is made locally with the final `Status: PASS`
   report and the correct remediation commit field.
7. The maintainer pushes that release-candidate commit.
8. GitHub Actions and CodeQL run on that pushed commit.
9. If CodeQL reports findings, they are fixed, the report is updated with the
   security finding and new remediation commit, and a new release-candidate
   commit is made.
10. If GitHub checks are clean, tag that same already-pushed commit. Do not make
    a final report-only commit immediately before tagging, because that creates
    a new commit that GitHub has not checked yet.

The final release evidence lives in:

```text
security/pentest/<version>.md
```

The report must summarize:

- supplied `PENTEST.md` findings;
- local security-gate results;
- release-blocking fixes;
- scheduled follow-up releases for accepted non-blocking findings.
- CodeQL findings only when GitHub reports them after the pushed
  release-candidate commit.

## Required Pentest Report Fields

Each report must include:

```text
Status: DRAFT|PASS
Reviewed-Commit: TBD|<40 hex commit>
Remediation-Commit: TBD|<40 hex commit>
Tester: <name or team>
Scope:
Date: <date or TBD>
Summary:
Remediation:
Retest:
Verification:
Decision:
Result:
```

Report statuses:

- `DRAFT`: pentest input has been incorporated, but more retest or release
  work is expected.
- `PASS`: the pentest and retest are clean, release-blocking findings are
  fixed, and the report is ready to be included in a release-candidate commit.

`scripts/validate-release-candidate.sh` accepts draft and pass states for the
local release-candidate workflow.

`scripts/validate-release-readiness.sh` is intentionally stricter than the draft
format. It accepts only `Status: PASS` plus exact reviewed and remediation
commit hashes.

When used for the final tag decision, `scripts/validate-release-readiness.sh`
must be run on the same commit that was already pushed and green in GitHub. If a
CodeQL finding creates another fix, update the report with that finding and the
new remediation commit, push that new release-candidate commit, wait for green
GitHub checks, and only then tag it.
