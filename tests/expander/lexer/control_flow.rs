use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_if_block() {
    let input = "\
{% if product.count == 0 %}
Out of stock
{% endif %}";
    let mut l = Lexer::from(input);
    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::If, "if".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "product".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "count".to_string(), start_line, start_col),
        Token::new(TT::Equals, "==".to_string(), start_line, start_col),
        Token::new(TT::Integer, "0".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "Out of stock".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::EndIf, "endif".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::EOF, "".to_string(), start_line, start_col),
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
