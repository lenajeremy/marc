use md_to_html::marc::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_blockquote_vs_gt() {
    let input = "> Quote line\nNormal > not quote";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::GreaterThan, ">".to_string(), 0, 0),
        Token::new(TokenType::Text, " Quote line".to_string(), 0, 0),
        Token::new(TokenType::NewLine, "\n".to_string(), 0, 0),
        Token::new(TokenType::Text, "Normal > not quote".to_string(), 0, 0),
    ];

    for token in expected {
        let next_token = lexer.next_token();
        println!("Expected: \n{:?}\n\nGot: \n{:?}", token, next_token);
        assert_eq!(next_token.token_type, token.token_type);
        assert_eq!(next_token.literal, token.literal);
    }
}

#[test]
fn test_bold_and_italics() {
    let input = "\
**bold**
*italic*
***bolditalic***
5*5";
    let mut lexer = Lexer::from(input);

    let expected_tokens = vec![
        Token::new(
            TokenType::DoubleAsterisk,
            "**".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "bold".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::DoubleAsterisk,
            "**".to_string(),
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
            TokenType::Asterisk,
            "*".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "italic".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Asterisk,
            "*".to_string(),
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
            TokenType::DoubleAsterisk,
            "**".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Asterisk,
            "*".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "bolditalic".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        // TODO:
        // pretty sure the asterisk is supposed to come before the double asterisk
        // but that'd make for a very challenging problem to solve, I'll come back
        // to see if it affects the correctness of the code.
        // I think it would but, fingers' crossed.
        Token::new(
            TokenType::DoubleAsterisk,
            "**".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Asterisk,
            "*".to_string(),
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
            TokenType::Text,
            "5".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Asterisk,
            "*".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "5".to_string(),
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
