use md_to_html::marc::{lexer::Lexer, token::TokenType};

#[test]
fn test_complex_mixed() {
    let input = "# Heading\n> Quote\n**bold** and *italic* with [link](url) and `code` ![img](src)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        TokenType::H1,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::GreaterThan,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::DoubleAsterisk,
        TokenType::Text,
        TokenType::DoubleAsterisk,
        TokenType::Text,
        TokenType::Asterisk,
        TokenType::Text,
        TokenType::Asterisk,
        TokenType::Text,
        TokenType::LeftBracket,
        TokenType::Text,
        TokenType::RightBracket,
        TokenType::LeftParen,
        TokenType::Text,
        TokenType::RightParen,
        TokenType::Text,
        TokenType::Backtick,
        TokenType::Text,
        TokenType::Backtick,
        TokenType::Text,
        TokenType::Exclamation,
        TokenType::LeftBracket,
        TokenType::Text,
        TokenType::RightBracket,
        TokenType::LeftParen,
        TokenType::Text,
        TokenType::RightParen,
        TokenType::EOF,
    ];

    for token_type in expected {
        let token = lexer.next_token();
        println!("{}, {:?}", token.literal, token.token_type);
        assert_eq!(token.token_type, token_type);
    }
}
