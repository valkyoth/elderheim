use alloc::vec::Vec;

use elderheim_core::{CompileLimits, Span};

use crate::{
    Basic1HirError, Basic1HirProgram, Basic1HirStatement, Basic1HirStatementKind, Basic1LineNumber,
    Basic1Token, Basic1TokenKind, build_basic1_hir, build_basic1_hir_with_limits,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1ParsedProgram<'source> {
    pub lines: Vec<Basic1ParsedLine<'source>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1ParsedLine<'source> {
    pub number: Basic1LineNumber,
    pub statement: Basic1ParsedStatement<'source>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Basic1ParsedStatement<'source> {
    Print(Basic1PrintStatement<'source>),
    End,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Basic1PrintStatement<'source> {
    pub items: Vec<Basic1PrintItem<'source>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic1PrintItem<'source> {
    pub text: &'source str,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1ParseErrorKind {
    Hir(Basic1HirError),
    EmptyStatement,
    UnsupportedStatement,
    ExpectedPrintItem,
    ExpectedPrintSeparator,
    UnsupportedPrintItem,
    InvalidStringLiteral,
    TrailingPrintSeparator,
    EndHasOperands,
    StatementAfterEnd,
    MissingFinalEnd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic1ParseError {
    pub kind: Basic1ParseErrorKind,
    pub line_number: Option<Basic1LineNumber>,
    pub span: Option<Span>,
}

pub fn parse_basic1_program(source: &str) -> Result<Basic1ParsedProgram<'_>, Basic1ParseError> {
    let hir = build_basic1_hir(source).map_err(frontend_error)?;
    parse_hir(hir)
}

pub fn parse_basic1_program_with_limits(
    source: &str,
    limits: CompileLimits,
) -> Result<Basic1ParsedProgram<'_>, Basic1ParseError> {
    let hir = build_basic1_hir_with_limits(source, limits).map_err(frontend_error)?;
    parse_hir(hir)
}

fn parse_hir(hir: Basic1HirProgram<'_>) -> Result<Basic1ParsedProgram<'_>, Basic1ParseError> {
    let mut lines = Vec::new();
    let mut saw_end = false;
    let mut final_marker = None;

    for line in hir.lines {
        let first_span = line.statement.tokens.first().map(|token| token.span);
        let last_span = line.statement.tokens.last().map(|token| token.span);
        if saw_end {
            return Err(parse_error(
                Basic1ParseErrorKind::StatementAfterEnd,
                line.number,
                first_span,
            ));
        }

        let statement = parse_statement(line.number, line.statement)?;
        saw_end = matches!(&statement, Basic1ParsedStatement::End);
        final_marker = Some((line.number, last_span));
        lines.push(Basic1ParsedLine {
            number: line.number,
            statement,
        });
    }

    if !saw_end {
        let (line_number, span) = final_marker
            .map(|(number, span)| (Some(number), span.map(|value| Span::point(value.end()))))
            .unwrap_or((None, None));
        return Err(Basic1ParseError {
            kind: Basic1ParseErrorKind::MissingFinalEnd,
            line_number,
            span,
        });
    }

    Ok(Basic1ParsedProgram { lines })
}

fn parse_statement<'source>(
    line_number: Basic1LineNumber,
    statement: Basic1HirStatement<'source>,
) -> Result<Basic1ParsedStatement<'source>, Basic1ParseError> {
    match statement.kind {
        Basic1HirStatementKind::Print => {
            parse_print(line_number, statement.tokens).map(Basic1ParsedStatement::Print)
        }
        Basic1HirStatementKind::End => parse_end(line_number, statement.tokens),
        _ => Err(parse_error(
            Basic1ParseErrorKind::UnsupportedStatement,
            line_number,
            statement.tokens.first().map(|token| token.span),
        )),
    }
}

fn parse_print<'source>(
    line_number: Basic1LineNumber,
    tokens: Vec<Basic1Token<'source>>,
) -> Result<Basic1PrintStatement<'source>, Basic1ParseError> {
    let mut tokens = tokens.into_iter();
    if tokens.next().is_none() {
        return Err(parse_error(
            Basic1ParseErrorKind::EmptyStatement,
            line_number,
            None,
        ));
    }

    let mut items = Vec::new();
    let Some(mut token) = tokens.next() else {
        return Ok(Basic1PrintStatement { items });
    };

    loop {
        if token.kind == Basic1TokenKind::Comma {
            return Err(parse_error(
                Basic1ParseErrorKind::ExpectedPrintItem,
                line_number,
                Some(token.span),
            ));
        }
        items.push(parse_print_item(line_number, token)?);

        let Some(separator) = tokens.next() else {
            break;
        };
        if separator.kind != Basic1TokenKind::Comma {
            return Err(parse_error(
                Basic1ParseErrorKind::ExpectedPrintSeparator,
                line_number,
                Some(separator.span),
            ));
        }
        let Some(next_item) = tokens.next() else {
            return Err(parse_error(
                Basic1ParseErrorKind::TrailingPrintSeparator,
                line_number,
                Some(separator.span),
            ));
        };
        token = next_item;
    }

    Ok(Basic1PrintStatement { items })
}

fn parse_print_item(
    line_number: Basic1LineNumber,
    token: Basic1Token<'_>,
) -> Result<Basic1PrintItem<'_>, Basic1ParseError> {
    if token.kind != Basic1TokenKind::StringLiteral {
        return Err(parse_error(
            Basic1ParseErrorKind::UnsupportedPrintItem,
            line_number,
            Some(token.span),
        ));
    }

    let text = token
        .lexeme
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| {
            parse_error(
                Basic1ParseErrorKind::InvalidStringLiteral,
                line_number,
                Some(token.span),
            )
        })?;
    Ok(Basic1PrintItem {
        text,
        span: token.span,
    })
}

fn parse_end(
    line_number: Basic1LineNumber,
    tokens: Vec<Basic1Token<'_>>,
) -> Result<Basic1ParsedStatement<'_>, Basic1ParseError> {
    let mut tokens = tokens.into_iter();
    if tokens.next().is_none() {
        return Err(parse_error(
            Basic1ParseErrorKind::EmptyStatement,
            line_number,
            None,
        ));
    }
    if let Some(token) = tokens.next() {
        return Err(parse_error(
            Basic1ParseErrorKind::EndHasOperands,
            line_number,
            Some(token.span),
        ));
    }
    Ok(Basic1ParsedStatement::End)
}

const fn frontend_error(error: Basic1HirError) -> Basic1ParseError {
    Basic1ParseError {
        kind: Basic1ParseErrorKind::Hir(error),
        line_number: None,
        span: None,
    }
}

const fn parse_error(
    kind: Basic1ParseErrorKind,
    line_number: Basic1LineNumber,
    span: Option<Span>,
) -> Basic1ParseError {
    Basic1ParseError {
        kind,
        line_number: Some(line_number),
        span,
    }
}

#[cfg(test)]
#[path = "parser/tests.rs"]
mod tests;
