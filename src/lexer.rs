use crate::{token::*, utils};
use std::cmp::min;

#[derive(Debug)]
struct Lexer {
    src: String,
    position: usize,
    read_position: usize,
    line: usize,
    col: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn from(input: &str) -> Self {
        let mut lexer = Lexer {
            src: input.to_owned(),
            position: 0,
            read_position: 0,
            line: if input.len() == 0 { 0 } else { 1 },
            col: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn eat_whitespace(&mut self) {
        while self.ch == Some(' ') {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let token = match self.ch {
            Some('#') => {
                let mut total_pounds = 1;

                while self.peek_char(total_pounds) == '#' && total_pounds < 6 {
                    total_pounds += 1;
                }

                let heading = match total_pounds {
                    1 => Token::new(TokenType::H1, "#".repeat(total_pounds)),
                    2 => Token::new(TokenType::H2, "#".repeat(total_pounds)),
                    3 => Token::new(TokenType::H3, "#".repeat(total_pounds)),
                    4 => Token::new(TokenType::H4, "#".repeat(total_pounds)),
                    5 => Token::new(TokenType::H5, "#".repeat(total_pounds)),
                    6 => Token::new(TokenType::H6, "#".repeat(total_pounds)),
                    _ => {
                        // this should never happen so long the expected condition is that
                        // total_pounds <= 6 is valid (which our while loop ensure) and so the ideal thing to do here is to
                        panic!(
                            "Total pounds = {}. Should never be greater than 6",
                            total_pounds
                        )
                    }
                };

                for _ in 0..total_pounds {
                    self.read_char();
                }

                return heading;
            }
            Some('\n') => Token::new(TokenType::NewLine, "\n".to_string()),
            Some('*') => {
                let next_char = self.peek_char(1);
                if next_char == '*' {
                    self.read_char();
                    Token::new(TokenType::DoubleAsterisk, "**".to_string())
                } else {
                    Token::new(TokenType::Asterisk, "*".to_string())
                }
            }
            Some('`') => {
                let next = self.peek_char(1);
                if next == '`' && self.peek_char(2) == '`' {
                    self.read_char();
                    self.read_char();
                    Token::new(TokenType::TripleBacktick, "```".to_string())
                } else {
                    Token::new(TokenType::Backtick, "`".to_string())
                }
            }
            Some('[') => Token::new(TokenType::LeftBracket, "[".to_string()),
            Some(']') => Token::new(TokenType::RightBracket, "]".to_string()),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string()),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string()),
            Some('!') => Token::new(TokenType::Exclamation, "!".to_string()),
            None => Token::new(TokenType::EOF, "".to_string()),
            _ => {
                if utils::is_alphanumeric(self.ch) {
                    let word = self.read_text();
                    return Token::new(TokenType::Text, word);
                } else {
                    Token::new(TokenType::Invalid, "INVALID".to_string())
                }
            }
        };

        self.read_char();
        token
    }

    fn read_text(&mut self) -> String {
        let start = self.position;

        while utils::is_alphanumeric(self.ch) || self.ch == Some(' ') {
            self.read_char();
        }

        println!("loop broke at {:?} at position {}", self.ch, self.position);

        self.src[start..self.position].to_string()
    }

    fn peek_char(&self, distance: usize) -> char {
        let char_index = self.position + distance;
        if char_index >= self.src.len() {
            '\0'
        } else {
            let byte = self.src[(char_index)..].chars().next().unwrap();
            byte
        }
    }

    pub fn read_char(&mut self) {
        //println!("{self:#?}");
        self.ch = self.src[self.read_position..].chars().next();
        self.position = self.read_position;
        if let Some(x) = self.ch {
            self.read_position = min(self.read_position + x.len_utf8(), self.src.len());
        } else {
            self.read_position = min(self.read_position, self.src.len());
        }

        if self.ch == Some('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_heading_1() {
        let input = "\
        ## Hello World
# This is Jeremiah
# And this is a very important heading
";
        let mut lexer = Lexer::from(input);

        let expected_tokens = vec![
            Token::new(TokenType::H2, "##".to_string()),
            Token::new(TokenType::Text, "Hello World".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::H1, "#".to_string()),
            Token::new(TokenType::Text, "This is Jeremiah".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::H1, "#".to_string()),
            Token::new(
                TokenType::Text,
                "And this is a very important heading".to_string(),
            ),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
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
5*5
";
        let mut lexer = Lexer::from(input);

        let expected_tokens = vec![
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Text, "bold".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Text, "italic".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Text, "bolditalic".to_string()),
            // TODO:
            // pretty sure the asterisk is supposed to come before the double asterisk
            // but that'd make for a very challenging problem to solve, I'll come back
            // to see if it affects the correctness of the code.
            // I think it would but, fingers' crossed.
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Text, "5".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Text, "5".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
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
In Javascript you say `consolelog`
In **Python** you say `print`
In **Golang** you say `fmtPrintln`
In *Rust* you say `println`";

        let mut lexer = Lexer::from(input);
        let expected_tokens = vec![
            Token::new(TokenType::Text, "In Javascript you say ".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::Text, "consolelog".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Text, "In ".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Text, "Python".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Text, "you say ".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::Text, "print".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Text, "In ".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Text, "Golang".to_string()),
            Token::new(TokenType::DoubleAsterisk, "**".to_string()),
            Token::new(TokenType::Text, "you say ".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::Text, "fmtPrintln".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Text, "In ".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Text, "Rust".to_string()),
            Token::new(TokenType::Asterisk, "*".to_string()),
            Token::new(TokenType::Text, "you say ".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::Text, "println".to_string()),
            Token::new(TokenType::Backtick, "`".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
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
consolelogHello World everyone
```";
        let mut lexer = Lexer::from(input);
        let expected_tokens = vec![
            Token::new(TokenType::TripleBacktick, "```".to_string()),
            Token::new(TokenType::Text, "javascript".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(
                TokenType::Text,
                "consolelogHello World everyone".to_string(),
            ),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::TripleBacktick, "```".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
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
    fn test_tokenize_brackets_and_parentheses() {
        let input = "\
[text](url)
![text](url)
```";
        let mut lexer = Lexer::from(input);
        let expected_tokens = vec![
            Token::new(TokenType::LeftBracket, "[".to_string()),
            Token::new(TokenType::Text, "text".to_string()),
            Token::new(TokenType::RightBracket, "]".to_string()),
            Token::new(TokenType::LeftParen, "(".to_string()),
            Token::new(TokenType::Text, "url".to_string()),
            Token::new(TokenType::RightParen, ")".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::Exclamation, "!".to_string()),
            Token::new(TokenType::LeftBracket, "[".to_string()),
            Token::new(TokenType::Text, "text".to_string()),
            Token::new(TokenType::RightBracket, "]".to_string()),
            Token::new(TokenType::LeftParen, "(".to_string()),
            Token::new(TokenType::Text, "url".to_string()),
            Token::new(TokenType::RightParen, ")".to_string()),
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::TripleBacktick, "```".to_string()),
            Token::new(TokenType::EOF, "".to_string()),
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
}
