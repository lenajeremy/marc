use md_to_html::marc::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_newline_tokenizing() {
    let input = "Line1\nLine2";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        TokenType::Text,
        TokenType::NewLine,
        TokenType::Text,
        TokenType::EOF,
    ];
    for token_type in expected {
        assert_eq!(lexer.next_token().token_type, token_type);
    }
}

#[test]
fn test_bare_text() {
    let input = "HEllo World\nSomething interesting";
    let mut l = Lexer::from(input);

    let expected_tokens = vec![
        Token::new(TokenType::Text, "HEllo World".to_string(), 0, 0),
        Token::new(TokenType::NewLine, "\n".to_string(), 0, 0),
        Token::new(TokenType::Text, "Something interesting".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for expected_token in expected_tokens {
        let token = l.next_token();
        println!(
            "expected: {:?}, got: {:?}",
            expected_token.token_type, token.token_type
        );
        assert_eq!(token.token_type, expected_token.token_type);
        assert_eq!(token.literal, expected_token.literal);
    }
}
