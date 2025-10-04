use std::str::Chars;

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: (usize, usize),
}

#[derive(Debug)]
pub enum TokenKind {
    Be,
    Mut,
    If,
    Else,
    While,
    // For,
    Fn,
    Return,

    Struct,
    Enum,

    Boolean(bool),

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
    String(String),
    Integer(i64),
    Float(f64),

    Op(Op),
}

#[derive(PartialEq, Debug)]
pub enum Op {
    Pipe,

    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    Equal,
    EqualTo,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    And,
    Or,
    Not,

    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    ShiftLeft,
    ShiftRight,
}

pub struct Source<'a> {
    iter: Chars<'a>,
    peeked: Option<char>,
    pub src: &'a str,
}

impl<'a> Source<'a> {
    pub fn new(input: &'a str) -> Self {
        Source {
            iter: input.chars(),
            peeked: None,
            src: input,
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        if self.peeked.is_none() {
            self.peeked = self.iter.next();
        }
        self.peeked
    }
}

impl Iterator for Source<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(peeked) = self.peeked.take() {
            Some(peeked)
        } else {
            self.iter.next()
        }
    }
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
