use super::{Basic1HirError, Basic1HirStatementKind, build_basic1_hir, render_basic1_hir_snapshot};

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
fn rejects_non_statement_hir_start() {
    assert_eq!(
        build_basic1_hir("10 X\n20 END\n").map(|_| ()),
        Err(Basic1HirError::UnsupportedStatementStart)
    );
}
