use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_comparator_tokens() {
    let input = "{{ a != b }}\n{{ a <= b }}\n{{ a >= b }}";
    let mut l = Lexer::from(input);
    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "a".to_string(), start_line, start_col),
        Token::new(TT::NeQual, "!=".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "b".to_string(), start_line, start_col),
        Token::new(TT::RightDoubleBrace, "}}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "a".to_string(), start_line, start_col),
        Token::new(TT::LeQual, "<=".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "b".to_string(), start_line, start_col),
        Token::new(TT::RightDoubleBrace, "}}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "a".to_string(), start_line, start_col),
        Token::new(TT::GreQual, ">=".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "b".to_string(), start_line, start_col),
        Token::new(TT::RightDoubleBrace, "}}".to_string(), start_line, start_col),
        Token::new(TT::EOF, "".to_string(), start_line, start_col),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token.token_type.as_string(), expected.token_type.as_string());
        assert_eq!(token.literal, expected.literal);
    }
}
