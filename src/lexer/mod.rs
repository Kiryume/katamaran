pub mod types;

use types::{Op, Source, StringParser, Token, TokenKind};

pub struct LexerCursor<'a> {
    source: Source<'a>,
    position: usize,
    column: usize,
    row: usize,
}

impl<'a> LexerCursor<'a> {
    pub fn new(input: &'a str) -> Self {
        LexerCursor {
            source: Source::new(input),
            position: 0,
            column: 0,
            row: 1,
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
        self.source.peek()
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.row, self.column)
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
        &self.source.src[start..self.position]
    }
}

impl Iterator for LexerCursor<'_> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_while(char::is_whitespace);
        let c = self.bump()?;
        let (line, column) = self.pos();
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
            '=' => {
                if self.peek() == Some('=') {
                    self.bump();
                    TokenKind::Op(Op::EqualTo)
                } else {
                    TokenKind::Op(Op::Equal)
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
                Some('>') => {
                    self.bump();
                    TokenKind::Op(Op::ShiftRight)
                }
                _ => TokenKind::Op(Op::GreaterThan),
            },
            '<' => match self.peek() {
                Some('=') => {
                    self.bump();
                    TokenKind::Op(Op::LessThanOrEqual)
                }
                Some('<') => {
                    self.bump();
                    TokenKind::Op(Op::ShiftLeft)
                }
                _ => TokenKind::Op(Op::LessThan),
            },
            '&' => {
                if self.peek() == Some('&') {
                    self.bump();
                    TokenKind::Op(Op::And)
                } else {
                    TokenKind::Op(Op::BitAnd)
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
                _ => TokenKind::Op(Op::BitOr),
            },
            '^' => TokenKind::Op(Op::BitXor),
            '~' => TokenKind::Op(Op::BitNot),
            '"' => {
                let mut string_parser = StringParser::new();
                let string_content = self.eat_while(|c| string_parser.condition(c));
                if self.peek() == Some('"') {
                    self.bump();
                    TokenKind::String(string_content.to_string())
                } else {
                    return None;
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
                        Err(_) => {
                            return Some(Err(format!(
                                "Invalid float literal '{}' at line {}, column {}",
                                number_str, line, column
                            )));
                        }
                    }
                } else {
                    match number_str.parse::<i64>() {
                        Ok(num) => TokenKind::Integer(num),
                        Err(_) => {
                            return Some(Err(format!(
                                "Invalid integer literal '{}' at line {}, column {}",
                                number_str, line, column
                            )));
                        }
                    }
                }
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident_str = c.to_string();
                ident_str.push_str(self.eat_while(|ch| ch.is_ascii_alphanumeric() || ch == '_'));
                match ident_str.as_str() {
                    "be" => TokenKind::Be,
                    "mut" => TokenKind::Mut,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "while" => TokenKind::While,
                    "fn" => TokenKind::Fn,
                    "return" => TokenKind::Return,
                    "true" => TokenKind::Boolean(true),
                    "false" => TokenKind::Boolean(false),
                    "struct" => TokenKind::Struct,
                    "enum" => TokenKind::Enum,
                    _ => TokenKind::Identifier(ident_str),
                }
            }
            _ => {
                return Some(Err(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    c, line, column
                )));
            }
        };
        Some(Ok(Token {
            kind: tokenskind,
            pos: (line, column),
        }))
    }
}
