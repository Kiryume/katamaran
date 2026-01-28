use std::num::{ParseFloatError, ParseIntError};

use derivative::Derivative;
use miette::Diagnostic;
use thiserror::Error;

use crate::types::SrcSpan;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Token {
    pub kind: TokenKind,
    #[derivative(Debug = "ignore")]
    pub span: SrcSpan,
}

#[derive(Debug)]
pub enum TokenKind {
    Let,
    Fn,

    Match,
    Arrow,

    Comma,
    Colon,
    SemiColon,

    LParen,
    RParen,
    LCurly,
    RCurly,
    LBracket,
    RBracket,

    Identifier(String),
    Boolean(bool),
    String(String),
    Integer(i64),
    Float(f64),

    Op(Op),
}

#[derive(Debug, Error, Diagnostic)]
pub enum LexerError {
    #[error("unexpected character {found:?}")]
    #[diagnostic(code(lexer::unexpected_character))]
    #[help("remove the character or extend the grammar to accept it")]
    UnexpectedCharacter {
        found: char,
        #[label("unexpected character")]
        span: SrcSpan,
    },
    #[error("invalid integer literal")]
    #[diagnostic(code(lexer::invalid_integer_literal))]
    InvalidIntegerLiteral {
        literal: String,
        #[label("invalid integer literal")]
        span: SrcSpan,
        #[source]
        source: ParseIntError,
    },
    #[error("invalid float literal")]
    #[diagnostic(code(lexer::invalid_float_literal))]
    InvalidFloatLiteral {
        literal: String,
        #[label("invalid float literal")]
        span: SrcSpan,
        #[source]
        source: ParseFloatError,
    },
    #[error("unterminated string literal")]
    #[diagnostic(code(lexer::unterminated_string))]
    #[help("add a closing quote to complete the string")]
    UnterminatedString {
        #[label("string literal starts here")]
        span: SrcSpan,
    },
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Pipe,

    Dot,

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    Equal,   // =
    EqualTo, // ==
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    And,
    Or,
    Not,
    // BitAnd,
    // BitOr,
    // BitXor,
    // BitNot,
    // ShiftLeft,
    // ShiftRight,
}

pub struct StringParser {
    skip_next: bool,
}

impl StringParser {
    pub fn new() -> Self {
        StringParser { skip_next: false }
    }

    pub fn condition(&mut self, ch: char) -> bool {
        if self.skip_next {
            self.skip_next = false;
            return true;
        }

        match ch {
            '\\' => {
                self.skip_next = true;
                true
            }
            '"' => false,
            _ => true,
        }
    }
}
