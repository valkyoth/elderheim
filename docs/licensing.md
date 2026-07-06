# Licensing

Elderheim source code is licensed under `EUPL-1.2`.

This document clarifies the boundary between the Elderheim compiler project and
the programs compiled with Elderheim.

## Project License Scope

The `EUPL-1.2` license applies to Elderheim itself unless a file states a
different license. This includes the Rust source code, documentation, scripts,
tests, examples, release tooling, and other repository files.

Contributions to Elderheim are expected to be compatible with `EUPL-1.2`.

## Compiled Program Ownership

Source programs compiled with Elderheim remain owned and licensed by their
original authors, companies, or rights holders.

Using Elderheim to compile a program does not by itself:

- transfer ownership of the source program to the Elderheim project;
- require the source program to be licensed under `EUPL-1.2`;
- require the generated object file or executable to be licensed under
  `EUPL-1.2`.

The generated output belongs to the owner of the input program, subject to that
program's own license and any third-party material that the program includes.

## Runtime and Startup Code Boundary

Some compiler outputs may eventually need Elderheim-provided runtime, startup,
or support code. Examples could include platform entry code, numeric helper
functions, input/output routines, or other support fragments required by an old
language profile.

If generated output includes Elderheim-provided runtime, startup, or support
code, that included Elderheim component remains covered by the license stated
for that component. Before Elderheim ships production generated executables that
embed such material, the project must document the exact output/runtime
licensing terms for those components.

The intended policy is that users can compile their own programs with
Elderheim without the Elderheim project license taking over the user's source
program or generated executable merely because Elderheim was used as the
compiler.

## Documentation and Examples

Elderheim-authored documentation and examples in this repository are part of the
project and are licensed under `EUPL-1.2` unless a file states otherwise.

Historical manuals, references, or third-party source material are not relicensed
by Elderheim. When the project documents legacy language behavior, it uses
Elderheim-authored descriptions and tracks provenance separately.

## Legal Note

This document is a project policy statement, not legal advice. Organizations
with strict licensing requirements should review the `EUPL-1.2` license text and
their own source-program licenses before release.
