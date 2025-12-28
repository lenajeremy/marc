use md_to_html::expander::lexer::Lexer;
use md_to_html::expander::token::{Token, TokenType as TT};

#[test]
fn test_mixed() {
    let input = "\
@import \"products.json\" as products;
@import \"person.json\" as me;

# Hi everyone. My name is {{ me.name }}
### And these are my products

{% for product in products %}
- {{ product.name }} {{ product.price }}
{% if product.total_available == 0 %}
Not Available
{% endif %}
{% endfor %}";

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
        Token::new(TT::Semicolon, ";".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::At, "@".to_string(), start_line, start_col),
        Token::new(TT::Import, "import".to_string(), start_line, start_col),
        Token::new(TT::DoubleQuote, "\"".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "person".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "json".to_string(), start_line, start_col),
        Token::new(TT::DoubleQuote, "\"".to_string(), start_line, start_col),
        Token::new(TT::As, "as".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "me".to_string(), start_line, start_col),
        Token::new(TT::Semicolon, ";".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TT::Text,
            "# Hi everyone. My name is ".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "me".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TT::Text,
            "### And these are my products".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::For, "for".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "product".to_string(), start_line, start_col),
        Token::new(TT::In, "in".to_string(), start_line, start_col),
        Token::new(
            TT::Identifier,
            "products".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "- ".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "product".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "name".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::Text, " ".to_string(), start_line, start_col),
        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "product".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "price".to_string(), start_line, start_col),
        Token::new(
            TT::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::If, "if".to_string(), start_line, start_col),
        Token::new(TT::Identifier, "product".to_string(), start_line, start_col),
        Token::new(TT::Dot, ".".to_string(), start_line, start_col),
        Token::new(
            TT::Identifier,
            "total_available".to_string(),
            start_col,
            start_line,
        ),
        Token::new(TT::Equals, "==".to_string(), start_line, start_col),
        Token::new(TT::Integer, "0".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::Text, "Not Available".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::EndIf, "endif".to_string(), start_line, start_col),
        Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col),
        Token::new(TT::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col),
        Token::new(TT::EndFor, "endfor".to_string(), start_line, start_col),
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
