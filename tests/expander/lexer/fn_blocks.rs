use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_fn_block_tokens() {
    let input = "{% fn add(a, b) %}\n{% return a + b %}\n{% endfn %}";
    let mut l = Lexer::from(input);
    let start_line = 0;
    let start_col = 0;

    let expected_tokens = vec![
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::Fn, "fn".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "add".to_string(), start_line, start_col),
        Token::new(TT::LeftParen, "(".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "a".to_string(), start_line, start_col),
        Token::new(TT::Comma, ",".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "b".to_string(), start_line, start_col),
        Token::new(TT::RightParen, ")".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::Return, "return".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "a".to_string(), start_line, start_col),
        Token::new(TT::Plus, "+".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "b".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::EndFn, "endfn".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::EOF, "".to_string(), start_line, start_col),
    ];

    for expected in expected_tokens {
        let token = l.next_token();
        assert_eq!(token.token_type.as_string(), expected.token_type.as_string());
        assert_eq!(token.literal, expected.literal);
    }
}
