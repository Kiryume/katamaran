pub mod types;

use std::{iter::Peekable, str::Chars};

use miette::Result;
use types::{LexerError, Op, StringParser, Token, TokenKind};
use unicode_ident::{is_xid_continue, is_xid_start};

use crate::types::SrcSpan;

pub type LexResult = Result<Token, LexerError>;

pub struct LexerCursor<'a> {
    source: Peekable<Chars<'a>>,
    position: usize,
    column: usize,
    row: usize,
    src: &'a str,
}

impl<'a> LexerCursor<'a> {
    pub fn new(input: &'a str) -> Self {
        LexerCursor {
            source: input.chars().peekable(),
            position: 0,
            column: 0,
            row: 1,
            src: input,
        }
    }

    pub fn bump(&mut self) -> Option<char> {
        let ch = self.source.next();
        if let Some(c) = ch {
            if c == '\n' {
                self.row += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
            self.position += c.len_utf8()
        }
        ch
    }

    pub fn peek(&mut self) -> Option<char> {
        self.source.peek().copied()
    }

    fn eat_while<F>(&mut self, mut condition: F) -> &'a str
    where
        F: FnMut(char) -> bool,
    {
        let start = self.position;
        while let Some(c) = self.peek() {
            if condition(c) {
                self.bump();
            } else {
                break;
            }
        }
        &self.src[start..self.position]
    }
}

impl Iterator for LexerCursor<'_> {
    type Item = LexResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_while(char::is_whitespace);
        let token_start = self.position;
        let c = self.bump()?;
        let tokenskind = match c {
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LCurly,
            '}' => TokenKind::RCurly,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '+' => TokenKind::Op(Op::Plus),
            '-' => TokenKind::Op(Op::Minus),
            '*' => TokenKind::Op(Op::Multiply),
            '/' => TokenKind::Op(Op::Divide),
            '%' => TokenKind::Op(Op::Modulo),
            '.' => TokenKind::Op(Op::Dot),
            '=' => {
                // if self.peek() == Some('=') {
                //     self.bump();
                //     TokenKind::Op(Op::EqualTo)
                // } else {
                //     TokenKind::Op(Op::Equal)
                // }
                match self.peek() {
                    Some('=') => {
                        self.bump();
                        TokenKind::Op(Op::EqualTo)
                    }
                    Some('>') => {
                        self.bump();
                        TokenKind::Arrow
                    }
                    _ => TokenKind::Op(Op::Equal),
                }
            }
            '!' => {
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Op(Op::NotEqual)
                } else {
                    TokenKind::Op(Op::Not)
                }
            }
            '>' => match self.peek() {
                Some('=') => {
                    self.bump();
                    TokenKind::Op(Op::GreaterThanOrEqual)
                }
                _ => TokenKind::Op(Op::GreaterThan),
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.bump();
                    TokenKind::Op(Op::LessThanOrEqual)
                }
                _ => TokenKind::Op(Op::LessThan),
            },
            '&' => {
                if self.peek() == Some('&') {
                    self.bump();
                    TokenKind::Op(Op::And)
                } else {
                    let span = SrcSpan {
                        start: token_start,
                        end: token_start + c.len_utf8(),
                    };
                    return Some(Err(LexerError::UnexpectedCharacter { found: c, span }));
                }
            }
            '|' => match self.peek() {
                Some('|') => {
                    self.bump();
                    TokenKind::Op(Op::Or)
                }
                Some('>') => {
                    self.bump();
                    TokenKind::Op(Op::Pipe)
                }
                _ => {
                    let span = SrcSpan {
                        start: token_start,
                        end: token_start + c.len_utf8(),
                    };
                    return Some(Err(LexerError::UnexpectedCharacter { found: c, span }));
                }
            },
            '"' => {
                let mut string_parser = StringParser::new();
                let string_content = self.eat_while(|c| string_parser.condition(c));
                if self.peek() == Some('"') {
                    self.bump();
                    TokenKind::String(string_content.to_string())
                } else {
                    let span = SrcSpan {
                        start: token_start,
                        end: self.position,
                    };
                    return Some(Err(LexerError::UnterminatedString { span }));
                }
            }
            c if c.is_ascii_digit() => {
                let mut number_str = c.to_string();
                number_str.push_str(self.eat_while(|ch| ch.is_ascii_digit()));
                if self.peek() == Some('.') {
                    number_str.push('.');
                    self.bump();
                    number_str.push_str(self.eat_while(|ch| ch.is_ascii_digit()));
                    match number_str.parse::<f64>() {
                        Ok(num) => TokenKind::Float(num),
                        Err(source) => {
                            let span = SrcSpan {
                                start: token_start,
                                end: self.position,
                            };
                            return Some(Err(LexerError::InvalidFloatLiteral {
                                literal: number_str,
                                span,
                                source,
                            }));
                        }
                    }
                } else {
                    match number_str.parse::<i64>() {
                        Ok(num) => TokenKind::Integer(num),
                        Err(source) => {
                            let span = SrcSpan {
                                start: token_start,
                                end: self.position,
                            };
                            return Some(Err(LexerError::InvalidIntegerLiteral {
                                literal: number_str,
                                span,
                                source,
                            }));
                        }
                    }
                }
            }
            c if is_xid_start(c) || c == '_' => {
                let mut ident_str = c.to_string();
                ident_str.push_str(self.eat_while(|ch| is_xid_continue(ch) || ch == '_'));
                match ident_str.as_str() {
                    "let" => TokenKind::Let,
                    "fn" => TokenKind::Fn,
                    "match" => TokenKind::Match,
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    _ => TokenKind::Identifier(ident_str),
                }
            }
            _ => {
                let span = SrcSpan {
                    start: token_start,
                    end: token_start + c.len_utf8(),
                };
                return Some(Err(LexerError::UnexpectedCharacter { found: c, span }));
            }
        };
        let span_len = self.position - token_start;
        let span = SrcSpan {
            start: token_start,
            end: token_start + span_len,
        };
        Some(Ok(Token {
            kind: tokenskind,
            span,
        }))
    }
}
