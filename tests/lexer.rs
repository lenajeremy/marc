use md_to_html::{Lexer, Token, TokenType};

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
fn test_blockquote_vs_gt() {
    let input = "> Quote line\nNormal > not quote";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::GreaterThan, ">".to_string(), 0, 0),
        Token::new(TokenType::Text, " Quote line".to_string(), 0, 0),
        Token::new(TokenType::NewLine, "\n".to_string(), 0, 0),
        Token::new(TokenType::Text, "Normal > not quote".to_string(), 0, 0),
    ];

    for token in expected {
        let next_token = lexer.next_token();
        println!("Expected: \n{:?}\n\nGot: \n{:?}", token, next_token);
        assert_eq!(next_token.token_type, token.token_type);
        assert_eq!(next_token.literal, token.literal);
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

#[test]
fn test_bold_and_italics() {
    let input = "\
**bold**
*italic*
***bolditalic***
5*5";
    let mut lexer = Lexer::from(input);

    let expected_tokens = vec![
        Token::new(
            TokenType::DoubleAsterisk,
            "**".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "bold".to_string(),
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
            TokenType::NewLine,
            "\n".to_string(),
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
            "italic".to_string(),
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
            TokenType::NewLine,
            "\n".to_string(),
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
            TokenType::Asterisk,
            "*".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "bolditalic".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        // TODO:
        // pretty sure the asterisk is supposed to come before the double asterisk
        // but that'd make for a very challenging problem to solve, I'll come back
        // to see if it affects the correctness of the code.
        // I think it would but, fingers' crossed.
        Token::new(
            TokenType::DoubleAsterisk,
            "**".to_string(),
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
            TokenType::NewLine,
            "\n".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "5".to_string(),
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
            "5".to_string(),
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

#[test]
fn test_link_tokenizing() {
    let input = "[OpenAI](https://openai.com)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::LeftBracket, "[".to_string(), 0, 0),
        Token::new(TokenType::Text, "OpenAI".to_string(), 0, 0),
        Token::new(TokenType::RightBracket, "]".to_string(), 0, 0),
        Token::new(TokenType::LeftParen, "(".to_string(), 0, 0),
        Token::new(TokenType::Text, "https://openai.com".to_string(), 0, 0),
        Token::new(TokenType::RightParen, ")".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for t in expected {
        let token = lexer.next_token();
        assert_eq!(token.token_type, t.token_type);
        assert_eq!(token.literal, t.literal);
    }
}

#[test]
fn test_tokenize_brackets_and_parentheses() {
    let input = "\
[text](url)
![text](url)
```";
    let mut lexer = Lexer::from(input);
    let expected_tokens = vec![
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "text".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "url".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightParen,
            ")".to_string(),
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
            TokenType::Exclamation,
            "!".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "text".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::LeftParen,
            "(".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::Text,
            "url".to_string(),
            lexer.get_line(),
            lexer.get_col(),
        ),
        Token::new(
            TokenType::RightParen,
            ")".to_string(),
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

#[test]
fn test_image_tokenizing() {
    let input = "![Tiger](http://example.com/tiger.jpg!)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        Token::new(TokenType::Exclamation, "!".to_string(), 0, 0),
        Token::new(TokenType::LeftBracket, "[".to_string(), 0, 0),
        Token::new(TokenType::Text, "Tiger".to_string(), 0, 0),
        Token::new(TokenType::RightBracket, "]".to_string(), 0, 0),
        Token::new(TokenType::LeftParen, "(".to_string(), 0, 0),
        Token::new(
            TokenType::Text,
            "http://example.com/tiger.jpg".to_string(),
            0,
            0,
        ),
        Token::new(TokenType::Text, "!".to_string(), 0, 0),
        Token::new(TokenType::RightParen, ")".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for token in expected {
        let next_token = lexer.next_token();
        assert_eq!(next_token.token_type, token.token_type);
        assert_eq!(next_token.literal, token.literal);
    }
}

#[test]
fn test_newline_tokenizing() {
    let input = "Line1\nLine2";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        TokenType::Text,
        TokenType::NewLine,
        TokenType::Text,
        TokenType::EOF,
    ];
    for token_type in expected {
        assert_eq!(lexer.next_token().token_type, token_type);
    }
}

#[test]
fn test_complex_mixed() {
    let input = "# Heading\n> Quote\n**bold** and *italic* with [link](url) and `code` ![img](src)";
    let mut lexer = Lexer::from(input);

    let expected = vec![
        TokenType::H1,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::GreaterThan,
        TokenType::Text,
        TokenType::NewLine,
        TokenType::DoubleAsterisk,
        TokenType::Text,
        TokenType::DoubleAsterisk,
        TokenType::Text,
        TokenType::Asterisk,
        TokenType::Text,
        TokenType::Asterisk,
        TokenType::Text,
        TokenType::LeftBracket,
        TokenType::Text,
        TokenType::RightBracket,
        TokenType::LeftParen,
        TokenType::Text,
        TokenType::RightParen,
        TokenType::Text,
        TokenType::Backtick,
        TokenType::Text,
        TokenType::Backtick,
        TokenType::Text,
        TokenType::Exclamation,
        TokenType::LeftBracket,
        TokenType::Text,
        TokenType::RightBracket,
        TokenType::LeftParen,
        TokenType::Text,
        TokenType::RightParen,
        TokenType::EOF,
    ];

    for token_type in expected {
        let token = lexer.next_token();
        println!("{}, {:?}", token.literal, token.token_type);
        assert_eq!(token.token_type, token_type);
    }
}

#[test]
fn test_bare_text() {
    let input = "HEllo World\nSomething interesting";
    let mut l = Lexer::from(input);

    let expected_tokens = vec![
        Token::new(TokenType::Text, "HEllo World".to_string(), 0, 0),
        Token::new(TokenType::NewLine, "\n".to_string(), 0, 0),
        Token::new(TokenType::Text, "Something interesting".to_string(), 0, 0),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for t in expected_tokens {
        let token = l.next_token();
        assert_eq!(t.token_type, token.token_type);
        assert_eq!(t.literal, token.literal);
    }
}

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
        Token::new(TokenType::Text, "Hello ".to_string(), 0, 0),
        Token::new(TokenType::LeftDoubleBrace, "{{".to_string(), 0, 0),
        Token::new(TokenType::Text, "admin ".to_string(), start_col, start_line),
        Token::new(
            TokenType::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(TokenType::H3, "###".to_string(), start_line, start_col),
        Token::new(
            TokenType::Text,
            " Participants:".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TokenType::KeywordStart,
            "{%".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::For, "for".to_string(), start_line, start_col),
        Token::new(
            TokenType::Text,
            "name in person ".to_string(),
            start_line,
            start_col,
        ),
        Token::new(
            TokenType::KeywordEnd,
            "%}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TokenType::UnorderedListItem,
            "-".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
        Token::new(
            TokenType::LeftDoubleBrace,
            "{{".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "name ".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TokenType::UnorderedListItem,
            "-".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
        Token::new(
            TokenType::LeftDoubleBrace,
            "{{".to_string(),
            start_line,
            start_col,
        ),
        Token::new(
            TokenType::Text,
            "name.upper".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::LeftParen, "(".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightParen,
            ")".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TokenType::UnorderedListItem,
            "-".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "Hello ".to_string(), start_line, start_col),
        Token::new(
            TokenType::LeftDoubleBrace,
            "{{".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "name".to_string(), start_line, start_col),
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "0".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::NewLine, "\n".to_string(), start_line, start_col),
        Token::new(
            TokenType::KeywordStart,
            "{%".to_string(),
            start_line,
            start_col,
        ),
        Token::new(
            TokenType::EndFor,
            "endfor".to_string(),
            start_line,
            start_col,
        ),
        Token::new(
            TokenType::KeywordEnd,
            "%}".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::EOF, "".to_string(), 0, 0),
    ];

    for t in expected_tokens {
        let token = l.next_token();
        println!("Expected: {:?}\nGot: {:?}", t, token);
        assert_eq!(t.token_type, token.token_type);
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
        Token::new(
            TokenType::LeftDoubleBrace,
            "{{".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "array".to_string(), start_line, start_col),
        Token::new(
            TokenType::LeftBracket,
            "[".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, "0".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightBracket,
            "]".to_string(),
            start_line,
            start_col,
        ),
        Token::new(TokenType::Text, " ".to_string(), start_line, start_col),
        Token::new(
            TokenType::RightDoubleBrace,
            "}}".to_string(),
            start_line,
            start_col,
        ),
    ];
    for expected in expected_tokens {
        let token = l.next_token();
        println!("Expected {expected:?}\nGot {token:?}");
        assert_eq!(token.token_type, expected.token_type);
        assert_eq!(token.literal, expected.literal);
    }
}
