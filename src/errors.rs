use crate::token::{Token, TokenKind};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum MonkeyError {
    #[error("expected next token to be \"{:?}\", got \"{:?}\" instead", .0, .1)]
    UnexpectedToken(TokenKind, Token),
    #[error("invalid token \"{:?}\"", .0)]
    InvalidToken(Token),
}
