use types::ast::{ExpressionStatement, Literal, LiteralExpr, ReturnStatement};
pub use types::{
    Parser,
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

impl Parser {
    pub fn parse_statements(&mut self) -> Option<Vec<Statement>> {
        let mut statements = Vec::new();
        loop {
            if self.tokenstream.peek().is_none() {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        Some(statements)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        let peeked = self.peek_token()?;
        let stmt = match peeked.kind {
            TokenTreeKind::Be => Statement::Be(self.parse_be_statement()?),
            TokenTreeKind::Return => Statement::Return(self.parse_return_statement()?),
            _ => Statement::Expression(self.parse_expression_statement()?),
        };
        Some(stmt)
    }

    fn parse_be_statement(&mut self) -> Option<BeStatement> {
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
        let ident: Ident = self.parse_ident()?;
        if !peek_is!(self.tokenstream, TokenTreeKind::Op(ref op) if op == &Op::Equal) {
            self.errors.push(format!(
                "Expected '=' after identifier at line {}, column {}",
                ident.pos.0, ident.pos.1
            ));
            return None;
        } else {
            self.tokenstream.next();
        }
        let value: Expression = self.parse_expression()?;
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

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        let pos = self.tokenstream.next()?.pos;
        let expr: Expression = self.parse_expression()?;
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
        Some(ReturnStatement { expr, pos })
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        let pos = self.tokenstream.peek()?.pos;
        let expr: Expression = self.parse_expression()?;
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

    fn parse_expression(&mut self) -> Option<Expression> {
        let peeked = self.peek_token()?;
        let expr = match &peeked.kind {
            TokenTreeKind::Identifier(_) => Expression::Ident(self.parse_ident()?),
            TokenTreeKind::Integer(_)
            | TokenTreeKind::Float(_)
            | TokenTreeKind::Boolean(_)
            | TokenTreeKind::String(_) => Expression::Literal(self.parse_literal()?),
            _ => {
                let (peeked_string, line, col) =
                    (format!("{:?}", peeked.kind), peeked.pos.0, peeked.pos.1);
                self.errors.push(format!(
                    "Unexpected token '{}' at line {}, column {}",
                    peeked_string, line, col
                ));
                self.tokenstream.next();
                return None;
            }
        };
        Some(expr)
    }

    fn parse_ident(&mut self) -> Option<Ident> {
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
            None
        }
    }

    fn parse_literal(&mut self) -> Option<LiteralExpr> {
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
