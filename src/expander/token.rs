#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Text,
    Identifier,
    NewLine,
    Integer,

    // Symbols
    LeftDoubleBrace,  // {{
    RightDoubleBrace, // }}
    KeywordStart,     // {%
    KeywordEnd,       // %}
    LeftBracket,      // {
    RightBracket,     // }
    LeftParen,        // (
    RightParen,       // )
    Exclamation,      // !
    Assign,           // =
    SingleQuote,      // '
    DoubleQuote,      // "
    Semicolon,        // ;

    // Math Operators
    Asterisk,     // *
    ForwardSlash, // /
    Minus,        // -
    Plus,         // +

    // Comparators
    GreaterThan, // >
    LessThan,    // <
    GreQual,     // >=
    LeQual,      // <=
    Equals,      // ==

    // Keywords
    If,
    EndIf,
    For,
    EndFor,
    In,
    Import,
    Include,
    True,
    False,
    As,

    // Others
    Dot,
    EOF,
    At,
}

impl TokenType {
    pub fn as_string(&self) -> String {
        match self {
            TokenType::Text => "TEXT_TYPE".to_string(),
            TokenType::Identifier => "IDENTIFIER".to_string(),
            TokenType::GreaterThan => ">".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::NewLine => "\n".to_string(),
            TokenType::LeftBracket => "[".to_string(),
            TokenType::RightBracket => "]".to_string(),
            TokenType::LeftParen => "(".to_string(),
            TokenType::RightParen => ")".to_string(),
            TokenType::Exclamation => "!".to_string(),
            TokenType::LeftDoubleBrace => "{{".to_string(),
            TokenType::RightDoubleBrace => "}}".to_string(),
            TokenType::KeywordStart => "{%".to_string(),
            TokenType::KeywordEnd => "%}".to_string(),
            TokenType::EOF => "EOF".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::EndIf => "endif".to_string(),
            TokenType::For => "for".to_string(),
            TokenType::EndFor => "endfor".to_string(),
            TokenType::In => "in".to_string(),
            TokenType::Import => "import".to_string(),
            TokenType::Include => "include".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::False => "false".to_string(),
            TokenType::Assign => "=".to_string(),
            TokenType::As => "as".to_string(),
            TokenType::Asterisk => "*".to_string(),
            TokenType::ForwardSlash => "/".to_string(),
            TokenType::Minus => "-".to_string(),
            TokenType::Plus => "+".to_string(),
            TokenType::LessThan => "<".to_string(),
            TokenType::GreQual => ">=".to_string(),
            TokenType::LeQual => "<=".to_string(),
            TokenType::Equals => "==".to_string(),
            TokenType::At => "@".to_string(),
            TokenType::Integer => "INTEGER".to_string(),
            TokenType::SingleQuote => "'".to_string(),
            TokenType::DoubleQuote => "\"".to_string(),
            TokenType::Semicolon => ";".to_string(),
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
