use super::{
    Basic1HirError, Basic1HirStatementKind, build_basic1_hir, build_basic1_hir_with_limits,
    render_basic1_hir_snapshot,
};
use crate::LineTableErrorKind;
use alloc::{format, string::String};
use elderheim_core::CompileLimits;

#[test]
fn builds_source_shaped_hir_for_print_end_program() -> Result<(), Basic1HirError> {
    let program = build_basic1_hir("10 PRINT \"HELLO\"\n20 END\n")?;
    assert_eq!(program.lines.len(), 2);

    let mut lines = program.lines.iter();
    let first = lines.next();
    let second = lines.next();

    assert_eq!(
        first.map(|line| line.statement.kind),
        Some(Basic1HirStatementKind::Print)
    );
    assert_eq!(first.map(|line| line.statement.expressions.len()), Some(1));
    assert_eq!(
        second.map(|line| line.statement.kind),
        Some(Basic1HirStatementKind::End)
    );
    assert_eq!(second.map(|line| line.statement.expressions.len()), Some(0));
    Ok(())
}

#[test]
fn hir_snapshot_for_basic1_hello_is_stable() -> Result<(), Basic1HirError> {
    let program = build_basic1_hir(include_str!(
        "../../../../../examples/dartmouth-basic-1/hello.bas"
    ))?;
    assert_eq!(
        render_basic1_hir_snapshot(&program),
        "basic1-hir\nline 10 Print tokens=2 expressions=1\n  expr \"HELLO FROM DARTMOUTH BASIC 1\"\nline 20 End tokens=1 expressions=0\n"
    );
    Ok(())
}

#[test]
fn hir_snapshot_for_basic1_branch_is_stable() -> Result<(), Basic1HirError> {
    let program = build_basic1_hir(include_str!(
        "../../../../../examples/dartmouth-basic-1/branch.bas"
    ))?;
    assert_eq!(
        render_basic1_hir_snapshot(&program),
        "basic1-hir\nline 10 Let tokens=4 expressions=1\n  expr 3\nline 20 If tokens=6 expressions=1\n  expr X < 5\nline 30 Print tokens=2 expressions=1\n  expr \"FIVE OR MORE\"\nline 40 GoTo tokens=3 expressions=0\nline 50 Print tokens=2 expressions=1\n  expr \"LESS THAN FIVE\"\nline 60 End tokens=1 expressions=0\n"
    );
    Ok(())
}

#[test]
fn hir_snapshot_escapes_string_literal_control_bytes() -> Result<(), Basic1HirError> {
    let program = build_basic1_hir("10 PRINT \"\u{1b}\u{7}\u{0}\"\n20 END\n")?;
    let snapshot = render_basic1_hir_snapshot(&program);

    assert_eq!(
        snapshot,
        "basic1-hir\nline 10 Print tokens=2 expressions=1\n  expr \"\\x1b\\x07\\x00\"\nline 20 End tokens=1 expressions=0\n"
    );
    assert!(!snapshot.contains('\u{1b}'));
    assert!(!snapshot.contains('\u{7}'));
    assert!(!snapshot.contains('\u{0}'));
    Ok(())
}

#[test]
fn hir_snapshot_escapes_non_ascii_and_formatting_characters() -> Result<(), Basic1HirError> {
    let program = build_basic1_hir("10 PRINT \"\u{85}\u{e9}\u{202e}\u{2066}\"\n20 END\n")?;
    let snapshot = render_basic1_hir_snapshot(&program);

    assert_eq!(
        snapshot,
        "basic1-hir\nline 10 Print tokens=2 expressions=1\n  expr \"\\x85\\xe9\\u{202e}\\u{2066}\"\nline 20 End tokens=1 expressions=0\n"
    );
    for character in ['\u{85}', '\u{e9}', '\u{202e}', '\u{2066}'] {
        assert!(!snapshot.contains(character));
    }
    Ok(())
}

#[test]
fn builds_hir_for_every_committed_basic1_example() -> Result<(), Basic1HirError> {
    for source in [
        include_str!("../../../../../examples/dartmouth-basic-1/hello.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/arithmetic.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/for-next.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/branch.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/def-function.bas"),
        include_str!("../../../../../examples/dartmouth-basic-1/read-data.bas"),
    ] {
        let program = build_basic1_hir(source)?;
        assert!(!program.lines.is_empty());
    }
    Ok(())
}

#[test]
fn hir_budget_is_bounded_by_source_and_line_limits() -> Result<(), Basic1HirError> {
    let limits = CompileLimits::with_source_limits(4096, 128);
    let mut source = String::new();

    for line_number in 1_u32..=128 {
        source.push_str(&format!("{line_number} PRINT 1,2,3,4\n"));
    }

    let program = build_basic1_hir_with_limits(&source, limits)?;
    let mut token_count = 0_usize;
    for line in &program.lines {
        token_count = token_count.saturating_add(line.statement.tokens.len());
    }

    assert_eq!(program.lines.len(), limits.max_lines);
    assert!(source.len() <= limits.max_source_bytes);
    assert!(token_count <= limits.max_source_bytes);
    Ok(())
}

#[test]
fn hir_rejects_source_before_program_wide_token_growth() {
    let limits = CompileLimits::with_source_limits(32, 128);
    let error =
        build_basic1_hir_with_limits("10 PRINT 1,2,3,4,5,6,7,8,9\n20 END\n", limits).map(|_| ());

    assert_eq!(
        error,
        Err(Basic1HirError::LineTable(crate::LineTableError {
            kind: LineTableErrorKind::SourceTooLarge,
            physical_line_index: 0,
        }))
    );
}

#[test]
fn rejects_non_statement_hir_start() {
    assert_eq!(
        build_basic1_hir("10 X\n20 END\n").map(|_| ()),
        Err(Basic1HirError::UnsupportedStatementStart)
    );
}
