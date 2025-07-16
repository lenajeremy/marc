use crate::token::{self, TokenType};

pub const KEYWORDS: [&str; 9] = [
    "for", "endfor", "in", "includes", "import", "if", "endif", "true", "false",
];

pub fn is_alphanumeric(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_alphanumeric(),
        None => false,
    }
}

pub fn is_numeric(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => ch.is_numeric(),
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

pub fn is_inline_token(ch: Option<char>) -> bool {
    match ch {
        Some('*') | Some('[') | Some(']') | Some('(') | Some(')') | Some('!') | Some('`')
        | Some('{') | Some('%') | Some('}') => true,
        _ => false,
    }
}

pub fn is_keyword(t: &str) -> bool {
    KEYWORDS.contains(&t)
}
