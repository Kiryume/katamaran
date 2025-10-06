use types::ast::{ExpressionStatement, Literal, LiteralExpr, ReturnStatement};
pub use types::{
    Parse, Parser,
    ast::{BeStatement, Expression, Ident, Pos, Statement},
};

use crate::{lexer::types::Op, tokentree::TokenTreeKind};

pub mod types;
#[macro_export]
macro_rules! peek_is {
    ($iter:expr, $($pat:tt)+) => {
        $iter.peek().is_some_and(|tok| matches!(tok.kind, $($pat)+))
    };
}
#[macro_export]
macro_rules! extract {
    ($enum:ident :: $variant:ident ( $($pat:pat),* ) = $expr:expr, $name:ident) => {{
        if let $enum::$variant($($pat),*) = $expr {
            $name
        } else {
            unreachable!(
                "Expected {}::{} variant",
                stringify!($enum),
                stringify!($variant)
            )
        }
    }};

    ($enum:ident :: $variant:ident { $($pat:tt)* } = $expr:expr, $name:ident) => {{
        if let $enum::$variant { $($pat)* } = $expr {
            $name
        } else {
            unreachable!(
                "Expected {}::{} variant",
                stringify!($enum),
                stringify!($variant)
            )
        }
    }};
}
impl Parse<Vec<Statement>> for Parser {
    fn parse(&mut self) -> Option<Vec<Statement>> {
        let mut root = Vec::new();
        loop {
            if self.tokenstream.peek().is_none() {
                break;
            }
            if let Some(stmt) = self.parse() {
                root.push(stmt);
            }
        }
        Some(root)
    }
}

impl Parse<Statement> for Parser {
    fn parse(&mut self) -> Option<Statement> {
        let peeked = self.peek_token()?;
        let stmt = match peeked.kind {
            TokenTreeKind::Be => Statement::Be(self.parse()?),
            TokenTreeKind::Return => Statement::Return(self.parse()?),
            _ => Statement::Expression(self.parse()?),
        };
        Some(stmt)
    }
}

impl Parse<BeStatement> for Parser {
    fn parse(&mut self) -> Option<BeStatement> {
        let pos = self.tokenstream.next().unwrap().pos;
        let is_mut = if peek_is!(self.tokenstream, TokenTreeKind::Mut) {
            self.tokenstream.next();
            true
        } else {
            false
        };
        if !peek_is!(self.tokenstream, TokenTreeKind::Identifier(_)) {
            self.errors.push(format!(
                "Expected identifier after 'be' at line {}, column {}",
                pos.0, pos.1
            ));
            return None;
        }
        let ident: Ident = self.parse()?;
        if !peek_is!(self.tokenstream, TokenTreeKind::Op(ref op) if op == &Op::Equal) {
            self.errors.push(format!(
                "Expected '=' after identifier at line {}, column {}",
                ident.pos.0, ident.pos.1
            ));
            return None?;
        } else {
            self.tokenstream.next();
        }
        let value: Expression = self.parse()?;
        if !peek_is!(self.tokenstream, TokenTreeKind::SemiColon) {
            self.errors.push(format!(
                "Expected ';' after expression at line {}, column {}",
                value.pos().0,
                value.pos().1
            ));
            return None;
        } else {
            self.tokenstream.next();
        }
        Some(BeStatement {
            ident,
            value,
            is_mut,
            pos,
        })
    }
}

impl Parse<ReturnStatement> for Parser {
    fn parse(&mut self) -> Option<ReturnStatement> {
        let pos = self.tokenstream.next()?.pos;
        let value: Expression = self.parse()?;
        if !peek_is!(self.tokenstream, TokenTreeKind::SemiColon) {
            self.errors.push(format!(
                "Expected ';' after expression at line {}, column {}",
                value.pos().0,
                value.pos().1
            ));
            return None;
        } else {
            self.tokenstream.next();
        }
        Some(ReturnStatement { value, pos })
    }
}

impl Parse<ExpressionStatement> for Parser {
    fn parse(&mut self) -> Option<ExpressionStatement> {
        let pos = self.tokenstream.peek()?.pos;
        let expr: Expression = self.parse()?;
        if !peek_is!(self.tokenstream, TokenTreeKind::SemiColon) {
            self.errors.push(format!(
                "Expected ';' after expression at line {}, column {}",
                expr.pos().0,
                expr.pos().1
            ));
            return None;
        } else {
            self.tokenstream.next();
        }
        Some(ExpressionStatement { expr, pos })
    }
}

impl Parse<Expression> for Parser {
    fn parse(&mut self) -> Option<Expression> {
        let peeked = self.peek_token()?;
        let expr = match &peeked.kind {
            TokenTreeKind::Identifier(_) => Expression::Ident(self.parse()?),
            TokenTreeKind::Integer(_)
            | TokenTreeKind::Float(_)
            | TokenTreeKind::Boolean(_)
            | TokenTreeKind::String(_) => Expression::Literal(self.parse()?),
            _ => {
                let (peeked_string, line, col) =
                    (format!("{:?}", peeked.kind), peeked.pos.0, peeked.pos.1);
                self.errors.push(format!(
                    "Unexpected token '{}' at line {}, column {}",
                    peeked_string, line, col
                ));
                return None;
            }
        };
        Some(expr)
    }
}

impl Parse<Ident> for Parser {
    fn parse(&mut self) -> Option<Ident> {
        let token = self.tokenstream.next()?;
        if let TokenTreeKind::Identifier(name) = &token.kind {
            Some(Ident {
                name: name.to_string(),
                pos: token.pos,
            })
        } else {
            self.errors.push(format!(
                "Expected identifier, found '{:?}' at line {}, column {}",
                token.kind, token.pos.0, token.pos.1
            ));
            return None;
        }
    }
}

impl Parse<LiteralExpr> for Parser {
    fn parse(&mut self) -> Option<LiteralExpr> {
        let token = self.tokenstream.next()?;
        let literal = match &token.kind {
            TokenTreeKind::Integer(value) => Literal::Int(*value),
            TokenTreeKind::Float(value) => Literal::Float(*value),
            TokenTreeKind::Boolean(value) => Literal::Bool(*value),
            TokenTreeKind::String(value) => Literal::Str(value.clone()),
            _ => {
                self.errors.push(format!(
                    "Expected literal, found '{:?}' at line {}, column {}",
                    token.kind, token.pos.0, token.pos.1
                ));
                return None;
            }
        };
        Some(LiteralExpr {
            value: literal,
            pos: token.pos,
        })
    }
}
