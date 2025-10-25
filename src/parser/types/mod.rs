use std::{iter::Peekable, vec::IntoIter};

use crate::tokentree::TokenTree;

pub mod ast;

pub struct Parser {
    pub(super) tokenstream: Peekable<IntoIter<TokenTree>>,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(tokenstream: Vec<TokenTree>) -> Self {
        Self {
            tokenstream: tokenstream.into_iter().peekable(),
            errors: Vec::new(),
        }
    }

    pub fn peek_token(&mut self) -> Option<&TokenTree> {
        if self.tokenstream.peek().is_none() {
            self.errors.push("Unexpected EOF".to_string());
            return None;
        }
        self.tokenstream.peek()
    }
}

enum Precedence {
    Lowest = 1,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
}
