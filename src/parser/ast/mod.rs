use std::iter::Peekable;

use crate::{
    lexer::types::Op,
    tokentree::{TokenTree, TokenTreeKind},
};
pub use types::*;

pub mod types;
#[macro_export]
macro_rules! peek_is {
    ($iter:expr, $($pat:tt)+) => {
        $iter.peek().is_some_and(|tok| matches!(tok.kind, $($pat)+))
    };
}
#[macro_export]
macro_rules! extract {
    // Usage: extract_variant!(EnumType::Variant(inner_var) = expr)
    ($enum:ident :: $variant:ident ($var:ident) = $expr:expr) => {{
        if let $enum::$variant(inner) = $expr {
            inner
        } else {
            unreachable!(
                "Expected {}::{} variant",
                stringify!($enum),
                stringify!($variant)
            )
        }
    }};
}

pub trait Parse: Sized {
    fn parse(tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self, String>;

    fn next_token(
        tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<TokenTree, String> {
        tokenstream
            .next()
            .ok_or("End of file reached unexpectedly".to_string())
    }
    fn peek_token(
        tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>,
    ) -> Result<&TokenTree, String> {
        tokenstream
            .peek()
            .ok_or("End of file reached unexpectedly".to_string())
    }
}

impl Parse for Statement {
    fn parse(tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self, String> {
        let next = Statement::peek_token(tokenstream)?;
        let stmt = match next.kind {
            TokenTreeKind::Be => Statement::Let(BeStatement::parse(tokenstream)?),
            _ => todo!("Rest of token tree kinds"),
        };
        Ok(stmt)
    }
}

impl Parse for BeStatement {
    fn parse(tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self, String> {
        let pos = tokenstream.next().unwrap().pos;
        let is_mut = if peek_is!(tokenstream, TokenTreeKind::Mut) {
            tokenstream.next();
            true
        } else {
            false
        };
        if !peek_is!(tokenstream, TokenTreeKind::Identifier(_)) {
            return Err(format!(
                "Expected identifier after 'be' at line {}, column {}",
                pos.0, pos.1
            ));
        }
        let ident_token = tokenstream.next().unwrap();
        let ident = Ident {
            name: extract!(TokenTreeKind::Identifier(name) = &ident_token.kind).to_string(),
            pos: ident_token.pos,
        };
        if !peek_is!(tokenstream, TokenTreeKind::Op(ref op) if op == &Op::Equal) {
            return Err(format!(
                "Expected '=' after identifier at line {}, column {}",
                ident_token.pos.0, ident_token.pos.1
            ));
        } else {
            tokenstream.next();
        }
        let value = Expression::parse(tokenstream)?;
        Ok(BeStatement {
            ident,
            value,
            is_mut,
            pos,
        })
    }
}

impl Parse for Expression {
    fn parse(tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self, String> {
        let next = Expression::peek_token(tokenstream)?;
        let expr = match &next.kind {
            TokenTreeKind::Identifier(_) => {
                let ident = Ident::parse(tokenstream)?;
                Expression::Ident(ident)
            }
            _ => {
                return Err(format!(
                    "Unexpected token '{:?}' at line {}, column {}",
                    next.kind, next.pos.0, next.pos.1
                ));
            }
        };
        Ok(expr)
    }
}

impl Parse for Ident {
    fn parse(_tokenstream: &mut Peekable<impl Iterator<Item = TokenTree>>) -> Result<Self, String> {
        let token = Ident::next_token(_tokenstream)?;
        if let TokenTreeKind::Identifier(name) = &token.kind {
            Ok(Ident {
                name: name.to_string(),
                pos: token.pos,
            })
        } else {
            Err(format!(
                "Expected identifier, found '{:?}' at line {}, column {}",
                token.kind, token.pos.0, token.pos.1
            ))
        }
    }
}
