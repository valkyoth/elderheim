use alloc::vec::Vec;

use elderheim_core::{CompileLimits, Span};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1Keyword {
    Data,
    Def,
    End,
    Fn,
    For,
    Go,
    Goto,
    If,
    Let,
    Next,
    Print,
    Read,
    Step,
    Then,
    To,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1BuiltinFunction {
    Abs,
    Atn,
    Cos,
    Exp,
    Int,
    Log,
    Rnd,
    Sin,
    Sqr,
    Tan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1TokenKind {
    Keyword(Basic1Keyword),
    BuiltinFunction(Basic1BuiltinFunction),
    Identifier,
    UserFunctionIdentifier,
    Number,
    StringLiteral,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Equal,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    LeftParen,
    RightParen,
    Comma,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic1Token<'source> {
    pub kind: Basic1TokenKind,
    pub lexeme: &'source str,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Basic1LexErrorKind {
    InvalidIdentifier,
    InvalidNumber,
    UnterminatedString,
    TooManyTokens,
    UnknownCharacter,
    SpanOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Basic1LexError {
    pub kind: Basic1LexErrorKind,
    pub span: Span,
}

pub fn lex_basic1_statement(source: &str) -> Result<Vec<Basic1Token<'_>>, Basic1LexError> {
    lex_basic1_statement_with_limits(source, CompileLimits::DEFAULT)
}

pub fn lex_basic1_statement_with_limits(
    source: &str,
    limits: CompileLimits,
) -> Result<Vec<Basic1Token<'_>>, Basic1LexError> {
    let mut lexer = Lexer { source, offset: 0 };
    lexer.lex_statement(limits.max_tokens)
}

struct Lexer<'source> {
    source: &'source str,
    offset: usize,
}

impl<'source> Lexer<'source> {
    fn lex_statement(
        &mut self,
        max_tokens: usize,
    ) -> Result<Vec<Basic1Token<'source>>, Basic1LexError> {
        let mut tokens = Vec::new();

        while let Some(byte) = self.current_byte() {
            match byte {
                b' ' | b'\t' => self.advance_one()?,
                b'"' => push_token(&mut tokens, max_tokens, self.lex_string()?)?,
                b'.' if self.next_byte_is_digit() => {
                    push_token(&mut tokens, max_tokens, self.lex_number()?)?;
                }
                b'0'..=b'9' => push_token(&mut tokens, max_tokens, self.lex_number()?)?,
                b'A'..=b'Z' => push_token(&mut tokens, max_tokens, self.lex_word()?)?,
                b'+' => push_token(&mut tokens, max_tokens, self.single(Basic1TokenKind::Plus)?)?,
                b'-' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::Minus)?,
                )?,
                b'*' => push_token(&mut tokens, max_tokens, self.single(Basic1TokenKind::Star)?)?,
                b'/' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::Slash)?,
                )?,
                b'^' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::Caret)?,
                )?,
                b'=' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::Equal)?,
                )?,
                b'(' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::LeftParen)?,
                )?,
                b')' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::RightParen)?,
                )?,
                b',' => push_token(
                    &mut tokens,
                    max_tokens,
                    self.single(Basic1TokenKind::Comma)?,
                )?,
                b'<' | b'>' => push_token(&mut tokens, max_tokens, self.lex_relation()?)?,
                _ => return Err(self.error(Basic1LexErrorKind::UnknownCharacter, self.offset, 1)),
            }
        }

        Ok(tokens)
    }

    fn lex_string(&mut self) -> Result<Basic1Token<'source>, Basic1LexError> {
        let start = self.offset;
        self.advance_one()?;

        while let Some(byte) = self.current_byte() {
            self.advance_one()?;
            if byte == b'"' {
                return self.token(Basic1TokenKind::StringLiteral, start, self.offset);
            }
        }

        Err(self.error(
            Basic1LexErrorKind::UnterminatedString,
            start,
            self.offset.saturating_sub(start),
        ))
    }

    fn lex_number(&mut self) -> Result<Basic1Token<'source>, Basic1LexError> {
        let start = self.offset;
        self.consume_digits()?;

        if self.current_byte() == Some(b'.') {
            self.advance_one()?;
            self.consume_digits()?;
        }

        if matches!(self.current_byte(), Some(b'E')) {
            self.advance_one()?;
            if matches!(self.current_byte(), Some(b'+') | Some(b'-')) {
                self.advance_one()?;
            }
            if !self
                .current_byte()
                .is_some_and(|byte| byte.is_ascii_digit())
            {
                return Err(self.error(
                    Basic1LexErrorKind::InvalidNumber,
                    start,
                    self.offset.saturating_sub(start),
                ));
            }
            self.consume_digits()?;
        }

        self.token(Basic1TokenKind::Number, start, self.offset)
    }

    fn lex_word(&mut self) -> Result<Basic1Token<'source>, Basic1LexError> {
        let start = self.offset;
        while self
            .current_byte()
            .is_some_and(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
        {
            self.advance_one()?;
        }

        let word = self
            .source
            .get(start..self.offset)
            .ok_or_else(|| self.error(Basic1LexErrorKind::SpanOverflow, start, 0))?;
        let kind = classify_word(word)
            .ok_or_else(|| self.error(Basic1LexErrorKind::InvalidIdentifier, start, word.len()))?;
        self.token(kind, start, self.offset)
    }

    fn lex_relation(&mut self) -> Result<Basic1Token<'source>, Basic1LexError> {
        let start = self.offset;
        let first = self.current_byte();
        self.advance_one()?;
        let kind = match (first, self.current_byte()) {
            (Some(b'<'), Some(b'=')) => {
                self.advance_one()?;
                Basic1TokenKind::LessEqual
            }
            (Some(b'>'), Some(b'=')) => {
                self.advance_one()?;
                Basic1TokenKind::GreaterEqual
            }
            (Some(b'<'), Some(b'>')) => {
                self.advance_one()?;
                Basic1TokenKind::NotEqual
            }
            (Some(b'<'), _) => Basic1TokenKind::Less,
            (Some(b'>'), _) => Basic1TokenKind::Greater,
            _ => return Err(self.error(Basic1LexErrorKind::UnknownCharacter, start, 1)),
        };
        self.token(kind, start, self.offset)
    }

    fn single(&mut self, kind: Basic1TokenKind) -> Result<Basic1Token<'source>, Basic1LexError> {
        let start = self.offset;
        self.advance_one()?;
        self.token(kind, start, self.offset)
    }

    fn consume_digits(&mut self) -> Result<(), Basic1LexError> {
        while self
            .current_byte()
            .is_some_and(|byte| byte.is_ascii_digit())
        {
            self.advance_one()?;
        }
        Ok(())
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.offset).copied()
    }

    fn next_byte_is_digit(&self) -> bool {
        self.offset
            .checked_add(1)
            .and_then(|offset| self.source.as_bytes().get(offset))
            .is_some_and(u8::is_ascii_digit)
    }

    fn advance_one(&mut self) -> Result<(), Basic1LexError> {
        self.offset = self
            .offset
            .checked_add(1)
            .ok_or_else(|| self.error(Basic1LexErrorKind::SpanOverflow, self.source.len(), 0))?;
        Ok(())
    }

    fn token(
        &self,
        kind: Basic1TokenKind,
        start: usize,
        end: usize,
    ) -> Result<Basic1Token<'source>, Basic1LexError> {
        let span = span_from_bounds(start, end)?;
        let lexeme = self
            .source
            .get(start..end)
            .ok_or_else(|| self.error(Basic1LexErrorKind::SpanOverflow, start, 0))?;
        Ok(Basic1Token { kind, lexeme, span })
    }

    fn error(&self, kind: Basic1LexErrorKind, start: usize, len: usize) -> Basic1LexError {
        let span = span_from_start_len(start, len).unwrap_or_else(|_| Span::point(u32::MAX));
        Basic1LexError { kind, span }
    }
}

fn push_token<'source>(
    tokens: &mut Vec<Basic1Token<'source>>,
    max_tokens: usize,
    token: Basic1Token<'source>,
) -> Result<(), Basic1LexError> {
    if tokens.len() >= max_tokens {
        return Err(Basic1LexError {
            kind: Basic1LexErrorKind::TooManyTokens,
            span: token.span,
        });
    }
    tokens.push(token);
    Ok(())
}

fn classify_word(word: &str) -> Option<Basic1TokenKind> {
    keyword(word)
        .map(Basic1TokenKind::Keyword)
        .or_else(|| builtin_function(word).map(Basic1TokenKind::BuiltinFunction))
        .or_else(|| {
            if is_user_function_identifier(word) {
                Some(Basic1TokenKind::UserFunctionIdentifier)
            } else if is_variable_identifier(word) {
                Some(Basic1TokenKind::Identifier)
            } else {
                None
            }
        })
}

fn keyword(word: &str) -> Option<Basic1Keyword> {
    match word {
        "DATA" => Some(Basic1Keyword::Data),
        "DEF" => Some(Basic1Keyword::Def),
        "END" => Some(Basic1Keyword::End),
        "FN" => Some(Basic1Keyword::Fn),
        "FOR" => Some(Basic1Keyword::For),
        "GO" => Some(Basic1Keyword::Go),
        "GOTO" => Some(Basic1Keyword::Goto),
        "IF" => Some(Basic1Keyword::If),
        "LET" => Some(Basic1Keyword::Let),
        "NEXT" => Some(Basic1Keyword::Next),
        "PRINT" => Some(Basic1Keyword::Print),
        "READ" => Some(Basic1Keyword::Read),
        "STEP" => Some(Basic1Keyword::Step),
        "THEN" => Some(Basic1Keyword::Then),
        "TO" => Some(Basic1Keyword::To),
        _ => None,
    }
}

fn builtin_function(word: &str) -> Option<Basic1BuiltinFunction> {
    match word {
        "ABS" => Some(Basic1BuiltinFunction::Abs),
        "ATN" => Some(Basic1BuiltinFunction::Atn),
        "COS" => Some(Basic1BuiltinFunction::Cos),
        "EXP" => Some(Basic1BuiltinFunction::Exp),
        "INT" => Some(Basic1BuiltinFunction::Int),
        "LOG" => Some(Basic1BuiltinFunction::Log),
        "RND" => Some(Basic1BuiltinFunction::Rnd),
        "SIN" => Some(Basic1BuiltinFunction::Sin),
        "SQR" => Some(Basic1BuiltinFunction::Sqr),
        "TAN" => Some(Basic1BuiltinFunction::Tan),
        _ => None,
    }
}

fn is_variable_identifier(word: &str) -> bool {
    let mut bytes = word.bytes();
    let first = bytes.next();
    let second = bytes.next();
    let third = bytes.next();

    matches!(first, Some(b'A'..=b'Z'))
        && (second.is_none() || second.is_some_and(|byte| byte.is_ascii_digit()))
        && third.is_none()
}

fn is_user_function_identifier(word: &str) -> bool {
    let mut bytes = word.bytes();
    matches!(bytes.next(), Some(b'F'))
        && matches!(bytes.next(), Some(b'N'))
        && bytes.next().is_some_and(|byte| byte.is_ascii_uppercase())
        && bytes.next().is_none()
}

fn span_from_bounds(start: usize, end: usize) -> Result<Span, Basic1LexError> {
    let start = u32::try_from(start).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })?;
    let end = u32::try_from(end).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })?;
    Span::checked(start, end).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })
}

fn span_from_start_len(start: usize, len: usize) -> Result<Span, Basic1LexError> {
    let start = u32::try_from(start).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })?;
    let len = u32::try_from(len).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })?;
    Span::from_start_len(start, len).map_err(|_| Basic1LexError {
        kind: Basic1LexErrorKind::SpanOverflow,
        span: Span::point(u32::MAX),
    })
}

#[cfg(test)]
#[path = "lexer/tests.rs"]
mod tests;
