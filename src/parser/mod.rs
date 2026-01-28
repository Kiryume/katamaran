use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::lexer::types::{LexerError, Token, TokenKind};

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
    tok0: Token,
    tok1: Token,
    let_errors: Vec<LexerError>,
}
