#[derive(Debug, PartialEq, Clone)]
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
    Backtick,
    TripleBacktick,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Exclamation,

    UnorderedListItem,
    OrderedListItem,

    Text,
    NewLine,

    EOF,
}

impl TokenType {
    pub fn as_string(&self) -> String {
        match self {
            TokenType::H1 => "#".to_string(),
            TokenType::H2 => "##".to_string(),
            TokenType::H3 => "###".to_string(),
            TokenType::H4 => "####".to_string(),
            TokenType::H5 => "#####".to_string(),
            TokenType::H6 => "######".to_string(),
            TokenType::Text => "TEXT_TYPE".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::DoubleAsterisk => "**".to_string(),
            TokenType::GreaterThan => ">".to_string(),
            TokenType::NewLine => "\n".to_string(),
            TokenType::LeftBracket => "[".to_string(),
            TokenType::RightBracket => "]".to_string(),
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::Exclamation => "!".to_string(),
            TokenType::UnorderedListItem => "-".to_string(),
            TokenType::OrderedListItem => "1.".to_string(),
            TokenType::Backtick => "`".to_string(),
            TokenType::TripleBacktick => "```".to_string(),
            TokenType::EOF => "EOF".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
    pub start_line: usize,
    pub start_col: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: String,
        start_line: usize,
        start_col: usize,
    ) -> Token {
        let mut t = Token {
            token_type,
            literal,
            start_line,
            start_col,
        };
        t.start_col = start_col;
        t.start_line = start_line;
        t
    }
}
