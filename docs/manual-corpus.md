# Manual Corpus

Status: active 0.12.0 contract

Elderheim uses primary historical manuals to plan language support, but the
manual PDFs themselves are not committed unless their licensing allows it.

## Local Manual Paths

The 1.0 Dartmouth reference set is expected locally at:

```text
/home/eldryoth/Work/test/basicmanuals/first edition may 1964.pdf
/home/eldryoth/Work/test/basicmanuals/second edition october 1964.pdf
/home/eldryoth/Work/test/basicmanuals/196801_BASIC_4th_Edition_text.pdf
```

These paths are local provenance inputs. CI and contributors may not have them.
Validation must warn when they are missing, not fail solely because the local
manual files are absent.

## Committed Corpus

Committed corpus material must be either:

- Elderheim-authored documentation;
- small hand-written examples created for tests;
- metadata describing local source provenance.

The committed BASIC 1 corpus starts with:

- [Dartmouth BASIC 1](languages/dartmouth-basic-1.md)
- `examples/dartmouth-basic-1/`

## Validation

`scripts/validate-manual-corpus.sh` checks that the committed docs, example
manifest, and example files are present. It also reports local manual
availability without requiring the manual PDFs to be committed.
