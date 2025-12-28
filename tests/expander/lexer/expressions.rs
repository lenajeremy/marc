use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_array_access_expression() {
    let input = "{{ array[0] }}";
    let mut l = Lexer::from(input);
    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "array".to_string(), start_line, start_col),
        Token::new(TT::LeftBracket, "[".to_string(), start_line, start_col),
        Token::new(TT::Integer, "0".to_string(), start_line, start_col),
        Token::new(TT::RightBracket, "]".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
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
