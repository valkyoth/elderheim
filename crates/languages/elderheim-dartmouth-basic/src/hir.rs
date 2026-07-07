use alloc::{format, string::String, vec::Vec};
use core::fmt::Write as _;

use elderheim_core::{CompileLimits, Span};

use crate::{
    Basic1Keyword, Basic1LexError, Basic1LineNumber, Basic1Token, Basic1TokenKind, LineTableError,
    lex_basic1_statement_with_limits, parse_basic1_line_table_with_limits,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1HirProgram<'source> {
    pub lines: Vec<Basic1HirLine<'source>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1HirLine<'source> {
    pub number: Basic1LineNumber,
    pub statement: Basic1HirStatement<'source>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1HirStatement<'source> {
    pub kind: Basic1HirStatementKind,
    pub tokens: Vec<Basic1Token<'source>>,
    pub expressions: Vec<Basic1HirExpression<'source>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1HirExpression<'source> {
    pub tokens: Vec<Basic1Token<'source>>,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1HirStatementKind {
    Data,
    Def,
    End,
    For,
    GoTo,
    If,
    Let,
    Next,
    Print,
    Read,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1HirError {
    LineTable(LineTableError),
    Lex(Basic1LexError),
    EmptyStatement,
    UnsupportedStatementStart,
    ExpressionSpanOverflow,
}

pub fn build_basic1_hir(source: &str) -> Result<Basic1HirProgram<'_>, Basic1HirError> {
    build_basic1_hir_with_limits(source, CompileLimits::DEFAULT)
}

pub fn build_basic1_hir_with_limits(
    source: &str,
    limits: CompileLimits,
) -> Result<Basic1HirProgram<'_>, Basic1HirError> {
    let table =
        parse_basic1_line_table_with_limits(source, limits).map_err(Basic1HirError::LineTable)?;
    let mut lines = Vec::new();

    for entry in table.entries() {
        let tokens = lex_basic1_statement_with_limits(entry.statement, limits)
            .map_err(Basic1HirError::Lex)?;
        let kind = statement_kind(&tokens)?;
        let expressions = expression_sequences(kind, &tokens)?;
        lines.push(Basic1HirLine {
            number: entry.number,
            statement: Basic1HirStatement {
                kind,
                tokens,
                expressions,
            },
        });
    }

    Ok(Basic1HirProgram { lines })
}

pub fn render_basic1_hir_snapshot(program: &Basic1HirProgram<'_>) -> String {
    let mut output = String::new();
    output.push_str("basic1-hir\n");
    for line in &program.lines {
        output.push_str(&format!(
            "line {} {:?} tokens={} expressions={}\n",
            line.number.get(),
            line.statement.kind,
            line.statement.tokens.len(),
            line.statement.expressions.len()
        ));
        for expression in &line.statement.expressions {
            output.push_str("  expr");
            for token in &expression.tokens {
                output.push(' ');
                push_snapshot_escaped(&mut output, token.lexeme);
            }
            output.push('\n');
        }
    }
    output
}

fn push_snapshot_escaped(output: &mut String, text: &str) {
    for ch in text.chars() {
        match ch {
            '\u{0}'..='\u{1F}' | '\u{7F}' => {
                let _ = write!(output, "\\x{:02x}", u32::from(ch));
            }
            _ => output.push(ch),
        }
    }
}

fn statement_kind(tokens: &[Basic1Token<'_>]) -> Result<Basic1HirStatementKind, Basic1HirError> {
    let Some(token) = tokens.first() else {
        return Err(Basic1HirError::EmptyStatement);
    };

    match token.kind {
        Basic1TokenKind::Keyword(Basic1Keyword::Data) => Ok(Basic1HirStatementKind::Data),
        Basic1TokenKind::Keyword(Basic1Keyword::Def) => Ok(Basic1HirStatementKind::Def),
        Basic1TokenKind::Keyword(Basic1Keyword::End) => Ok(Basic1HirStatementKind::End),
        Basic1TokenKind::Keyword(Basic1Keyword::For) => Ok(Basic1HirStatementKind::For),
        Basic1TokenKind::Keyword(Basic1Keyword::Go | Basic1Keyword::Goto) => {
            Ok(Basic1HirStatementKind::GoTo)
        }
        Basic1TokenKind::Keyword(Basic1Keyword::If) => Ok(Basic1HirStatementKind::If),
        Basic1TokenKind::Keyword(Basic1Keyword::Let) => Ok(Basic1HirStatementKind::Let),
        Basic1TokenKind::Keyword(Basic1Keyword::Next) => Ok(Basic1HirStatementKind::Next),
        Basic1TokenKind::Keyword(Basic1Keyword::Print) => Ok(Basic1HirStatementKind::Print),
        Basic1TokenKind::Keyword(Basic1Keyword::Read) => Ok(Basic1HirStatementKind::Read),
        _ => Err(Basic1HirError::UnsupportedStatementStart),
    }
}

fn expression_sequences<'source>(
    kind: Basic1HirStatementKind,
    tokens: &[Basic1Token<'source>],
) -> Result<Vec<Basic1HirExpression<'source>>, Basic1HirError> {
    match kind {
        Basic1HirStatementKind::End
        | Basic1HirStatementKind::GoTo
        | Basic1HirStatementKind::Next => Ok(Vec::new()),
        Basic1HirStatementKind::Let | Basic1HirStatementKind::Def => sequences_after_equal(tokens),
        Basic1HirStatementKind::If => {
            sequence_between_keywords(tokens, Basic1Keyword::If, Basic1Keyword::Then)
        }
        Basic1HirStatementKind::For => for_sequences(tokens),
        Basic1HirStatementKind::Data
        | Basic1HirStatementKind::Print
        | Basic1HirStatementKind::Read => comma_sequences(tokens.iter().skip(1).copied()),
    }
}

fn sequences_after_equal<'source>(
    tokens: &[Basic1Token<'source>],
) -> Result<Vec<Basic1HirExpression<'source>>, Basic1HirError> {
    let after_equal = tokens
        .iter()
        .skip_while(|token| token.kind != Basic1TokenKind::Equal)
        .skip(1)
        .copied();
    comma_sequences(after_equal)
}

fn sequence_between_keywords<'source>(
    tokens: &[Basic1Token<'source>],
    start: Basic1Keyword,
    end: Basic1Keyword,
) -> Result<Vec<Basic1HirExpression<'source>>, Basic1HirError> {
    let inner = tokens
        .iter()
        .skip_while(|token| token.kind != Basic1TokenKind::Keyword(start))
        .skip(1)
        .take_while(|token| token.kind != Basic1TokenKind::Keyword(end))
        .copied();
    comma_sequences(inner)
}

fn for_sequences<'source>(
    tokens: &[Basic1Token<'source>],
) -> Result<Vec<Basic1HirExpression<'source>>, Basic1HirError> {
    let mut expressions = Vec::new();
    let initial = tokens
        .iter()
        .skip_while(|token| token.kind != Basic1TokenKind::Equal)
        .skip(1)
        .take_while(|token| token.kind != Basic1TokenKind::Keyword(Basic1Keyword::To))
        .copied();
    expressions.extend(comma_sequences(initial)?);

    let to_tail = tokens
        .iter()
        .skip_while(|token| token.kind != Basic1TokenKind::Keyword(Basic1Keyword::To))
        .skip(1)
        .take_while(|token| token.kind != Basic1TokenKind::Keyword(Basic1Keyword::Step))
        .copied();
    expressions.extend(comma_sequences(to_tail)?);
    let step_tail = tokens
        .iter()
        .skip_while(|token| token.kind != Basic1TokenKind::Keyword(Basic1Keyword::Step))
        .skip(1)
        .copied();
    expressions.extend(comma_sequences(step_tail)?);
    Ok(expressions)
}

fn comma_sequences<'source>(
    tokens: impl Iterator<Item = Basic1Token<'source>>,
) -> Result<Vec<Basic1HirExpression<'source>>, Basic1HirError> {
    let mut expressions = Vec::new();
    let mut current = Vec::new();
    for token in tokens {
        if token.kind == Basic1TokenKind::Comma {
            push_expression(&mut expressions, &mut current)?;
        } else {
            current.push(token);
        }
    }
    push_expression(&mut expressions, &mut current)?;
    Ok(expressions)
}

fn push_expression<'source>(
    expressions: &mut Vec<Basic1HirExpression<'source>>,
    current: &mut Vec<Basic1Token<'source>>,
) -> Result<(), Basic1HirError> {
    if current.is_empty() {
        return Ok(());
    }
    let span = expression_span(current)?;
    expressions.push(Basic1HirExpression {
        tokens: core::mem::take(current),
        span,
    });
    Ok(())
}

fn expression_span(tokens: &[Basic1Token<'_>]) -> Result<Span, Basic1HirError> {
    let Some(first) = tokens.first() else {
        return Err(Basic1HirError::ExpressionSpanOverflow);
    };
    let Some(last) = tokens.last() else {
        return Err(Basic1HirError::ExpressionSpanOverflow);
    };
    Span::checked(first.span.start(), last.span.end())
        .map_err(|_| Basic1HirError::ExpressionSpanOverflow)
}

#[cfg(test)]
#[path = "hir/tests.rs"]
mod tests;
