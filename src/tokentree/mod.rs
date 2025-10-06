use derivative::Derivative;

use crate::lexer::types::{Op, Token, TokenKind};

#[derive(PartialEq, Derivative)]
#[derivative(Debug)]
pub struct TokenTree {
    pub kind: TokenTreeKind,
    #[derivative(Debug = "ignore")]
    pub pos: (usize, usize),
}

#[derive(PartialEq, Debug)]
pub enum TokenTreeKind {
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

    Group {
        mode: GroupMode,
        children: Vec<TokenTree>,
    },

    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),

    Op(Op),
}

#[derive(PartialEq, Debug)]
pub enum GroupMode {
    Parens,
    Curly,
    Bracket,
}

impl TokenTree {
    pub fn parse_from_tokens(
        iter: &mut impl Iterator<Item = Token>,
    ) -> Result<Vec<TokenTree>, String> {
        let mut tokenstream = Vec::new();
        while let Some(token) = iter.next() {
            let tokentree = Self::token_to_tokentree(token, iter)?;
            tokenstream.push(tokentree);
        }
        Ok(tokenstream)
    }

    fn token_to_tokentree(
        token: Token,
        iter: &mut impl Iterator<Item = Token>,
    ) -> Result<TokenTree, String> {
        let kind = match token.kind {
            TokenKind::Be => TokenTreeKind::Be,
            TokenKind::Mut => TokenTreeKind::Mut,
            TokenKind::If => TokenTreeKind::If,
            TokenKind::Else => TokenTreeKind::Else,
            TokenKind::While => TokenTreeKind::While,
            TokenKind::Fn => TokenTreeKind::Fn,
            TokenKind::Return => TokenTreeKind::Return,

            TokenKind::Struct => TokenTreeKind::Struct,
            TokenKind::Enum => TokenTreeKind::Enum,

            TokenKind::Boolean(b) => TokenTreeKind::Boolean(b),

            TokenKind::Comma => TokenTreeKind::Comma,
            TokenKind::Colon => TokenTreeKind::Colon,
            TokenKind::SemiColon => TokenTreeKind::SemiColon,

            TokenKind::LParen | TokenKind::LCurly | TokenKind::LBracket => {
                let mode = TokenTree::token_to_groupmode(&token).unwrap();
                return Self::parse_group(mode, token, iter);
            }
            TokenKind::RParen | TokenKind::RCurly | TokenKind::RBracket => {
                return Err(format!(
                    "Unmatched closing bracket at line {}, column {}",
                    token.pos.0, token.pos.1
                ));
            }

            TokenKind::Identifier(s) => TokenTreeKind::Identifier(s),
            TokenKind::String(s) => TokenTreeKind::String(s),
            TokenKind::Integer(i) => TokenTreeKind::Integer(i),
            TokenKind::Float(f) => TokenTreeKind::Float(f),

            TokenKind::Op(op) => TokenTreeKind::Op(op),
        };
        Ok(TokenTree {
            kind,
            pos: token.pos,
        })
    }

    fn token_to_groupmode(token: &Token) -> Option<GroupMode> {
        match token.kind {
            TokenKind::RParen | TokenKind::LParen => Some(GroupMode::Parens),
            TokenKind::RCurly | TokenKind::LCurly => Some(GroupMode::Curly),
            TokenKind::RBracket | TokenKind::LBracket => Some(GroupMode::Bracket),
            _ => None,
        }
    }

    fn parse_group(
        mode: GroupMode,
        opening_token: Token,
        iter: &mut impl Iterator<Item = Token>,
    ) -> Result<TokenTree, String> {
        let mut children = Vec::new();
        let closing_token = TokenTree::token_to_groupmode(&opening_token);
        while let Some(token) = iter.next() {
            match token.kind {
                TokenKind::RParen | TokenKind::RCurly | TokenKind::RBracket => {
                    if TokenTree::token_to_groupmode(&token) == closing_token {
                        return Ok(TokenTree {
                            kind: TokenTreeKind::Group { mode, children },
                            pos: opening_token.pos,
                        });
                    } else {
                        return Err(format!(
                            "Mismatched closing bracket at line {}, column {}: expected {:?}, found {:?}",
                            token.pos.0, token.pos.1, closing_token, token.kind
                        ));
                    }
                }
                _ => {
                    let tokentree = Self::token_to_tokentree(token, iter)?;
                    children.push(tokentree);
                }
            }
        }
        Err(format!(
            "Unclosed bracket starting at line {}, column {}",
            opening_token.pos.0, opening_token.pos.1
        ))
    }
}
