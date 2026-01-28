use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Token {
    pub kind: TokenKind,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
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
