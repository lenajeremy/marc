use md_to_html::marc::{
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_tokenize_backticks() {
    let input = "\
In Javascript you say `console.log`
In **Python** you say `print`
In **Golang** you say `fmtPrintln`
In *Rust* you say `println`";

    let mut lexer = Lexer::from(input);
    let expected_tokens = vec![
        Token::new(
            TokenType::Text,
            "In Javascript you say ".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "console.log".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
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
            "In ".to_string(),
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
            TokenType::Text,
            "Python".to_string(),
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
            TokenType::Text,
            " you say ".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "print".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
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
            "In ".to_string(),
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
            TokenType::Text,
            "Golang".to_string(),
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
            TokenType::Text,
            " you say ".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "fmtPrintln".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
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
            "In ".to_string(),
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
            "Rust".to_string(),
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
            " you say ".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "println".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Backtick,
            "`".to_string(),
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

#[test]
fn test_tokenize_triple_backticks() {
    let input = "\
```javascript
let x = 15;
```";
    let mut lexer = Lexer::from(input);
    let expected_tokens = vec![
        Token::new(
            TokenType::TripleBacktick,
            "```".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "javascript".to_string(),
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
            "let x = 15;".to_string(),
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
            TokenType::TripleBacktick,
            "```".to_string(),
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
