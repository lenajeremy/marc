#[derive(Debug, PartialEq)]
pub enum TokenType {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Asterisk,
    DoubleAsterisk,
    GreaterThan,

    Text,
    NewLine,

    Invalid,
    EOF,
}

impl TokenType {
    fn as_string(&self) -> String {
        match self {
            Self::H1 => "#".to_string(),
            Self::H2 => "##".to_string(),
            Self::H3 => "###".to_string(),
            Self::H4 => "####".to_string(),
            Self::H5 => "#####".to_string(),
            Self::H6 => "######".to_string(),

            Self::Text => "TEXT_TYPE".to_string(),
            Self::Asterisk => "*".to_string(),
            Self::DoubleAsterisk => "**".to_string(),
            Self::GreaterThan => ">".to_string(),
            Self::NewLine => "\n".to_string(),

            Self::Invalid => "INVALID".to_string(),
            Self::EOF => "EOF".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
