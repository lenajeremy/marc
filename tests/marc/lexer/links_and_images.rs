use md_to_html::marc::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_link_tokenizing() {
    let input = "[OpenAI](https://openai.com)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::LeftBracket, "[".to_string(), 0, 0),
        Token::new(TokenType::Text, "OpenAI".to_string(), 0, 0),
        Token::new(TokenType::RightBracket, "]".to_string(), 0, 0),
        Token::new(TokenType::LeftParen, "(".to_string(), 0, 0),
        Token::new(TokenType::Text, "https://openai.com".to_string(), 0, 0),
        Token::new(TokenType::RightParen, ")".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for t in expected {
        let token = lexer.next_token();
        assert_eq!(token.token_type, t.token_type);
        assert_eq!(token.literal, t.literal);
    }
}

#[test]
fn test_tokenize_brackets_and_parentheses() {
    let input = "\
[text](url)
![text](url)
```";
    let mut lexer = Lexer::from(input);
    let expected_tokens = vec![
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "text".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "url".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightParen,
            ")".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::NewLine,
            "\n".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Exclamation,
            "!".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "text".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "url".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightParen,
            ")".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::NewLine,
            "\n".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::TripleBacktick,
            "```".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::EOF,
            "".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
    ];

    for expected_token in expected_tokens {
        let token = lexer.next_token();
        println!(
            "expected: {:?}, got: {:?}",
            expected_token.token_type, token.token_type
        );
        assert_eq!(token.token_type, expected_token.token_type);
        assert_eq!(token.literal, expected_token.literal);
    }
}

#[test]
fn test_image_tokenizing() {
    let input = "![Tiger](http://example.com/tiger.jpg!)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::Exclamation, "!".to_string(), 0, 0),
        Token::new(TokenType::LeftBracket, "[".to_string(), 0, 0),
        Token::new(TokenType::Text, "Tiger".to_string(), 0, 0),
        Token::new(TokenType::RightBracket, "]".to_string(), 0, 0),
        Token::new(TokenType::LeftParen, "(".to_string(), 0, 0),
        Token::new(
            TokenType::Text,
            "http://example.com/tiger.jpg".to_string(),
            0,
            7,
        ),
        Token::new(TokenType::Text, "!".to_string(), 0, 0),
        Token::new(TokenType::RightParen, ")".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for token in expected {
        let next_token = lexer.next_token();
        assert_eq!(next_token.token_type, token.token_type);
        assert_eq!(next_token.literal, token.literal);
    }
}
