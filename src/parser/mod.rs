use std::{iter::Peekable, vec::IntoIter};

use ast::{Parse, Statement};

use crate::tokentree::TokenTree;

mod ast;

pub struct Parser {
    tokenstream: Peekable<IntoIter<TokenTree>>,
}

impl Parser {
    pub fn new(tokenstream: Vec<TokenTree>) -> Self {
        Self {
            tokenstream: tokenstream.into_iter().peekable(),
        }
    }
    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut root = Vec::new();
        loop {
            if self.tokenstream.peek().is_none() {
                break;
            }
            let statement = Statement::parse(&mut self.tokenstream)?;
            root.push(statement);
        }

        Ok(root)
    }
}
