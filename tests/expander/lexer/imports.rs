use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_includes() {
    let input = "{% include 'footer.md' %}";
    let mut l = Lexer::from(input);

    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::Include, "include".to_string(), start_line, start_col),
        Token::new(TT::SingleQuote, "'".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "footer".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "md".to_string(), start_line, start_col),
        Token::new(TT::SingleQuote, "'".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        println!("Expected {expected:?}\nGot {token:?}");
        assert_eq!(
            token.token_type.as_string(),
            expected.token_type.as_string()
        );
        assert_eq!(token.literal, expected.literal);
    }
}

#[test]
fn test_imports() {
    let input = "@import \"products.json\" as products";

    let mut l = Lexer::from(input);

    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::At, "@".to_string(), start_line, start_col),
        Token::new(TT::Import, "import".to_string(), start_line, start_col),
        Token::new(TT::DoubleQuote, "\"".to_string(), start_line, start_col),
        Token::new(
            TT::Identifier,
            "products".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "json".to_string(), start_line, start_col),
        Token::new(TT::DoubleQuote, "\"".to_string(), start_line, start_col),
        Token::new(TT::As, "as".to_string(), start_line, start_col),
        Token::new(
            TT::Identifier,
            "products".to_string(),
            start_line,
            start_col,
        ),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        println!("Expected {expected:?}\nGot {token:?}");
        assert_eq!(
            token.token_type.as_string(),
            expected.token_type.as_string()
        );
        assert_eq!(token.literal, expected.literal);
    }
}
