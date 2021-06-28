use thiserror::Error;
use crate::token::{TokenKind, Token};

#[derive(Clone, Debug, Error)]
pub enum ParserError {
    #[error("expected next token to be \"{:?}\", got \"{:?}\" instead", .0, .1)]

    TokenInvalid(TokenKind, Token),
}