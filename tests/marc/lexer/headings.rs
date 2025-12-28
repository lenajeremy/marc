use md_to_html::marc::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_headings() {
    let input = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let mut lexer = Lexer::from(input);
    let expected = vec![
        TokenType::H1,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::H2,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::H3,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::H4,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::H5,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::H6,
        TokenType::Text,
        TokenType::EOF,
    ];
    for token_type in expected {
        let token = lexer.next_token();
        println!("expected {:?}, got {:?}", token_type, token);
        assert_eq!(token.token_type, token_type);
    }
}

#[test]
fn test_heading_1() {
    let input = "\
        ## Hello World
# This is Jeremiah
# And this is a very important heading
";
    let mut lexer = Lexer::from(input);

    let expected_tokens = vec![
        Token::new(
            TokenType::H2,
            "##".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            " Hello World".to_string(),
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
            TokenType::H1,
            "#".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            " This is Jeremiah".to_string(),
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
            TokenType::H1,
            "#".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            " And this is a very important heading".to_string(),
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
            TokenType::EOF,
            "".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
    ];

    for expected_token in expected_tokens {
        let token = lexer.next_token();
        println!("expected: {:?}\n got: {:?}", expected_token, token);
        assert_eq!(token.token_type, expected_token.token_type);
        assert_eq!(token.literal, expected_token.literal);
    }
}
