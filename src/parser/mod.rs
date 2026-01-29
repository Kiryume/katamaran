use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::lexer::{
    LexResult,
    types::{LexerError, Token, TokenKind},
};

#[derive(Debug, Error, Diagnostic)]
pub enum ParseError {
    #[error("unexpected token {found:?}")]
    #[diagnostic(code(parser::unexpected_token))]
    #[help("remove the token or extend the language to accept it")]
    UnexpectedCharacter {
        found: TokenKind,
        #[label("unexpected character")]
        span: SourceSpan,
    },
}

pub struct Parser<T: Iterator<Item = Result<Token, LexerError>>> {
    lexer: T,
    tok0: Option<Token>,
    tok1: Option<Token>,
    lex_errors: Vec<LexerError>,
}

impl<T: Iterator<Item = LexResult>> Parser<T> {
    fn new(lexer: T) -> Self {
        let mut p = Parser {
            lexer,
            tok0: None,
            tok1: None,
            lex_errors: vec![],
        };
        p.advance();
        p.advance();
        p
    }

    fn advance(&mut self) {
        let _ = self.next_tok();
    }
    fn next_tok(&mut self) -> Option<Token> {
        let t = self.tok0.take();
        let nxt;
        match self.lexer.next() {
            Some(Err(err)) => {
                nxt = None;
                self.lex_errors.push(err);
            }

            Some(Ok(tok)) => {
                nxt = Some(tok);
            }
            None => {
                nxt = None;
            }
        }
        self.tok0 = self.tok1.take();
        self.tok1 = nxt;
        t
    }
}
