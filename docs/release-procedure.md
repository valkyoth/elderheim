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

`Status: PASS` is allowed only after release-blocking pentest and CodeQL
findings are fixed, verified, and documented.
