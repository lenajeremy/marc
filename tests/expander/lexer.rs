use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_double_braces_with_keywords_and_lists() {
    let input = "\
Hello {{ admin }}
### Participants:
{% for name in person %}
- Hello {{ name }}
- Hello {{ name.upper() }}
- Hello {{ name[0] }}
{% endfor %}";
    let mut l = Lexer::from(input);
    let start_col = 0;
    let start_line = 0;

    let expected_tokens = vec![
        Token::new(TT::Text, "Hello ".to_string(), 0, 0),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), 0, 0),
        Token::new(TT::Identifier, "admin".to_string(), start_col, start_line),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TT::Text,
            "### Participants:".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::For, "for".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(TT::In, "in".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "person".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "- Hello ".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "- Hello ".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "upper".to_string(), start_line, start_col),
        Token::new(TT::LeftParen, "(".to_string(), start_line, start_col),
        Token::new(TT::RightParen, ")".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "- Hello ".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(TT::LeftBracket, "[".to_string(), start_line, start_col),
        Token::new(TT::Integer, "0".to_string(), start_line, start_col),
        Token::new(TT::RightBracket, "]".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::EndFor, "endfor".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::EOF, "".to_string(), 0, 0),
    ];

    for t in expected_tokens {
        let token = l.next_token();
        println!("Expected: {:?}\n\nGot: {:?}\n", t, token);
        assert_eq!(t.token_type.as_string(), token.token_type.as_string());
        assert_eq!(t.literal, token.literal);
    }
}

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
