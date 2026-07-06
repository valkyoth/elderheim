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
- the report must cover the exact release commit or say `Commit: TBD` while it
  is still a draft;
- CodeQL default setup findings must be reviewed and reflected in the report;
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
6. A release-candidate commit is made locally and pushed by the maintainer.
7. GitHub Actions and CodeQL run on that pushed commit.
8. If CodeQL reports findings, they are fixed, the report is updated, and a new
   release-candidate commit is made.
9. Only after CodeQL and the pentest report are final does the tag gate run.

The final release evidence lives in:

```text
security/pentest/<version>.md
```

The report must summarize:

- supplied `PENTEST.md` findings;
- CodeQL default setup findings;
- local security-gate results;
- release-blocking fixes;
- scheduled follow-up releases for accepted non-blocking findings.

## Required Pentest Report Fields

Each report must include:

```text
Status: DRAFT|PASS
Commit: TBD|<40 hex commit>
Tester: <name or team>
Date: <date or TBD>
CodeQL: TBD|PASS|FINDINGS
Scope:
Result:
```

Report statuses:

- `DRAFT`: pentest input has been incorporated, but more retest or release
  work is expected.
- `READY`: local full release checks passed and the commit is ready to push so
  GitHub Actions and CodeQL can review that exact state.
- `PASS`: release-blocking pentest and CodeQL findings are fixed, verified, and
  documented.

CodeQL statuses:

- `TBD`: CodeQL has not been checked for the current release work.
- `PENDING`: the ready commit is waiting for GitHub CodeQL results.
- `PASS`: CodeQL was reviewed and has no release-blocking findings.
- `FINDINGS`: CodeQL produced findings that are documented in the report.

`scripts/validate-release-candidate.sh` accepts draft and ready states for the
local ready-commit workflow.

`scripts/validate-release-readiness.sh` is intentionally stricter than the draft
format. It accepts only `Status: PASS`, an exact commit hash, and reviewed
CodeQL state.
