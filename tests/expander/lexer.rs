//#[test]
//fn test_double_braces_with_keywords_and_lists() {
//    let input = "\
//Hello {{ admin }}
//### Participants:
//{% for name in person %}
//- Hello {{ name }}
//- Hello {{ name.upper() }}
//- Hello {{ name[0] }}
//{% endfor %}";
//    let mut l = Lexer::from(input);
//    let start_col = 0;
//    let start_line = 0;
//
//    let expected_tokens = vec![
//        Token::new(TokenType::Text, "Hello ".to_string(), 0, 0),
//        Token::new(TokenType::LeftDoubleBrace, "{{".to_string(), 0, 0),
//        Token::new(TokenType::Text, "admin".to_string(), start_col, start_line),
//        Token::new(
//            TokenType::RightDoubleBrace,
//            "}}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(TokenType::H3, "###".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::Text,
//            " Participants:".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::KeywordStart,
//            "{%".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::For, "for".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::Text,
//            "name in person ".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(
//            TokenType::KeywordEnd,
//            "%}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::UnorderedListItem,
//            "-".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::LeftDoubleBrace,
//            "{{".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "name ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightDoubleBrace,
//            "}}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::UnorderedListItem,
//            "-".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::LeftDoubleBrace,
//            "{{".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(
//            TokenType::Text,
//            "name.upper".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::LeftParen, "(".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightParen,
//            ")".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightDoubleBrace,
//            "}}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::UnorderedListItem,
//            "-".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::LeftDoubleBrace,
//            "{{".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "name".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::LeftBracket,
//            "[".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "0".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightBracket,
//            "]".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightDoubleBrace,
//            "}}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::KeywordStart,
//            "{%".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(
//            TokenType::EndFor,
//            "endfor".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(
//            TokenType::KeywordEnd,
//            "%}".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::EOF, "".to_string(), 0, 0),
//    ];
//
//    for t in expected_tokens {
//        let token = l.next_token();
//        println!("Expected: {:?}\nGot: {:?}", t, token);
//        assert_eq!(t.token_type, token.token_type);
//        assert_eq!(t.literal, token.literal);
//    }
//}
//
//#[test]
//fn test_array_access_expression() {
//    let input = "{{ array[0] }}";
//    let mut l = Lexer::from(input);
//    let start_line = 0;
//    let start_col = 0;
//
//    let expected_tokens = vec![
//        Token::new(
//            TokenType::LeftDoubleBrace,
//            "{{".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "array".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::LeftBracket,
//            "[".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, "0".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightBracket,
//            "]".to_string(),
//            start_line,
//            start_col,
//        ),
//        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
//        Token::new(
//            TokenType::RightDoubleBrace,
//            "}}".to_string(),
//            start_line,
//            start_col,
//        ),
//    ];
//    for expected in expected_tokens {
//        let token = l.next_token();
//        println!("Expected {expected:?}\nGot {token:?}");
//        assert_eq!(token.token_type, expected.token_type);
//        assert_eq!(token.literal, expected.literal);
//    }
//}
