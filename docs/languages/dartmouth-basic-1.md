# Dartmouth BASIC 1

Status: active 0.13.0 corpus, frontend, and minimal parser reference

This document is Elderheim's own short reference for the Dartmouth BASIC First
Edition source-language corpus. It is based on the local May 1964 manual scan,
but the wording and examples here are written for Elderheim. The original scan
is not committed to this repository.

The 0.13.0 scope adds the first complete parser slice to the controlled corpus,
line table, lexer, and source-shaped HIR. It does not claim that Elderheim can
fully parse or compile all Dartmouth BASIC 1 programs yet.

## Source

Local reference path:

```text
/home/eldryoth/Work/test/basicmanuals/first edition may 1964.pdf
```

If this file is absent on a contributor or CI machine, corpus validation should
warn clearly and continue. The repository must remain buildable without
committing the manual PDF.

## Program Shape

A Dartmouth BASIC 1 source program is made from numbered lines. Each line has:

- a positive instruction number;
- a statement keyword or implied statement form;
- statement-specific operands.

The local corpus examples use increasing line numbers with gaps so later lines
can be inserted. The final source statement is `END`.

Teletype and time-sharing commands such as `HELLO`, `RUN`, `LIST`, `SAVE`, and
`STOP` are not compiler source statements for Elderheim. They belong to the
historical operating environment, not to the compiled language profile.

## Line Table Policy

Elderheim represents Dartmouth BASIC 1 source as an ordered table of numbered
source lines before later lexer and parser stages run.

The current policy is:

- every physical source line must be non-empty;
- every source line must begin with a positive decimal line number;
- line numbers must be one to five digits;
- line numbers must not begin with `0`;
- a line number must be followed by a space or tab before the statement text;
- the statement text after the line number must be non-empty;
- line numbers must be unique;
- line numbers must be strictly increasing in source order.

Malformed, duplicate, out-of-order, or empty numbered lines are rejected before
statement parsing. This gives later BASIC 1 parser work a stable source order
and branch-target model.

## Lexer Policy

Elderheim lexes Dartmouth BASIC 1 statement text after the line table has
removed the leading line number. Token spans are byte offsets relative to the
statement text, not the full source file.

The current lexer recognizes:

- BASIC 1 statement keywords such as `LET`, `PRINT`, `FOR`, `NEXT`, `GO`,
  `GOTO`, `IF`, `THEN`, `DEF`, `READ`, `DATA`, and `END`;
- built-in numeric function names from this reference;
- scalar variable identifiers made from one uppercase letter or one uppercase
  letter followed by one digit;
- user function identifiers of the form `FN` plus one uppercase letter;
- numeric literals, including decimals and `E` notation;
- quoted string literals;
- arithmetic operators, relation operators, commas, and parentheses.

Invalid identifier shapes, malformed exponent notation, unterminated strings,
source text over the compile byte limit, unknown characters, and token counts
over the compile limit are rejected by the lexer before later parser work.

## HIR Policy

Elderheim's first Dartmouth BASIC 1 HIR is source-shaped. It records:

- a program as ordered numbered lines;
- each line number and statement;
- the BASIC 1 statement family selected from the first statement token;
- the full token stream for the statement;
- expression-shaped token sequences for statement operands.

This HIR does not yet prove complete statement grammar or expression precedence.
Those checks begin in parser and semantic stops. The 0.12.0 HIR is a
stable typed boundary for parser work and report snapshots.

HIR snapshot output is diagnostic text, not source re-emission. Control bytes in
token lexemes are rendered as `\xNN` escape sequences so test logs, terminal
output, and review reports do not receive raw terminal-control characters from
untrusted BASIC source.

## Minimal Parser Policy

The `0.13.0` parser stop accepts the complete grammar claimed by this release:

- a program may contain blank `PRINT` statements;
- a `PRINT` may contain one or more quoted labels separated by commas;
- the program must end with exactly one operand-free `END` statement;
- no source statement may follow `END`;
- a source newline terminates a statement, while tokens on the same numbered
  line never act as an implicit terminator.

Leading, repeated, or trailing commas fail with a precise statement-relative
span. Adjacent labels without a comma, operands after `END`, statements after
`END`, and programs without a final `END` also fail closed. Numeric `PRINT`
items and other BASIC 1 statement families remain outside this parser stop and
are rejected explicitly until their scheduled parser versions implement their
complete grammar.

## Values And Formulas

The first-edition language uses numeric formulas built from:

- decimal numbers, including signed numbers;
- scientific notation with `E`;
- scalar variables named by one letter or one letter followed by one digit;
- arithmetic operators for addition, subtraction, multiplication, division,
  and exponentiation;
- parentheses;
- built-in numeric functions.

The current corpus reference tracks these built-in functions:

- `SIN`
- `COS`
- `TAN`
- `ATN`
- `EXP`
- `LOG`
- `SQR`
- `ABS`
- `RND`
- `INT`

Function arguments are numeric expressions. Trigonometric arguments are in
radians.

## Statements

### `LET`

`LET` assigns a numeric expression to a scalar variable.

```basic
10 LET X = (7+8)/3
20 END
```

### `PRINT`

`PRINT` writes one or more expressions or quoted labels. A blank `PRINT` line
is used as output spacing.

```basic
10 PRINT "HELLO"
20 PRINT X
30 END
```

### `FOR` / `NEXT`

`FOR` and `NEXT` repeat a block while changing a single-letter control
variable. The corpus includes the default step and explicit `STEP` forms.

```basic
10 FOR N = 1 TO 10
20 PRINT N
30 NEXT N
40 END
```

### `GO TO`

`GO TO` transfers control to another numbered source line. `GOTO` is tracked as
the same branch spelling for corpus examples.

```basic
10 GO TO 30
20 PRINT "SKIPPED"
30 END
```

### `IF ... THEN`

`IF ... THEN` compares numeric expressions and branches to a line number when
the relation is true. The corpus tracks equality, inequality, greater-than,
less-than, and the inclusive comparisons.

```basic
10 IF X < 5 THEN 40
20 PRINT "LARGE"
30 GO TO 50
40 PRINT "SMALL"
50 END
```

### `DEF`

`DEF` introduces a single-argument numeric function named with `FN` plus a
letter. The function body is a numeric expression.

```basic
10 DEF FNS(Z) = SIN(Z)
20 PRINT FNS(.5)
30 END
```

### `READ` / `DATA`

`DATA` stores numeric constants inside the program. `READ` consumes them in
order and assigns them to variables.

```basic
10 READ A, B
20 PRINT A, B
30 DATA 1, 2
40 END
```

## Examples

The controlled examples live in `examples/dartmouth-basic-1/`:

- `hello.bas`
- `print-labels.bas`
- `arithmetic.bas`
- `for-next.bas`
- `branch.bas`
- `def-function.bas`
- `read-data.bas`

The Dartmouth BASIC crate includes these files at test time and validates their
line numbering, lexer tokenization, HIR construction, final `END`,
source-statement families, and exclusion of historical session commands. The
minimal parser tests execute `hello.bas` and `print-labels.bas` as accepted
parser fixtures; later-statement examples remain fail-closed at this stop.

## Reserved Work

Dartmouth BASIC 2 and 4 have separate future corpus/reference documents.
Dartmouth BASIC 3 remains reserved because no official local manual is
available.
