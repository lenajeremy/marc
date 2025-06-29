use crate::token::{self, TokenType};

pub fn is_alphanumeric(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_alphanumeric(),
        None => false,
    }
}
pub fn is_block_level_token(t: token::TokenType) -> bool {
    match t {
        TokenType::TripleBacktick
        | TokenType::H1
        | TokenType::H2
        | TokenType::H3
        | TokenType::H4
        | TokenType::H5
        | TokenType::H6
        | TokenType::GreaterThan => true,
        _ => false,
    }
}
