use crate::token::*;
use std::cmp::min;

#[derive(Debug)]
struct Lexer {
    src: String,
    position: usize,
    read_position: usize,
    line: usize,
    col: usize,
    ch: char,
}

impl Lexer {
    pub fn from(input: &str) -> Self {
        let mut lexer = Lexer {
            src: input.to_owned(),
            position: 0,
            read_position: 0,
            line: if input.len() == 0 { 0 } else { 1 },
            col: 0,
            ch: '\0',
        };

        lexer.read_char();
        lexer
    }

    pub fn eat_whitespace(&mut self) {
        while self.ch == ' ' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '#' => {
                let mut total_pounds = 1;

                while self.peek_char(total_pounds - 1) == '#' && total_pounds < 6 {
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
            '\n' => Token::new(TokenType::NewLine, "\n".to_string()),
            _ => Token::new(TokenType::Invalid, "INVALID".to_string()),
        };

        self.read_char();
        token
    }

    fn peek_char(&self, distance: usize) -> char {
        let char_index = self.read_position + distance;
        if char_index >= self.src.len() {
            '\0'
        } else {
            let byte = self.src[(char_index + distance)..].chars().next().unwrap();
            byte
        }
    }

    pub fn read_char(&mut self) {
        self.ch = match self.src[self.read_position..].chars().next() {
            Some(x) => x,
            None => '\0',
        };
        self.position = self.read_position;
        self.read_position = min(self.read_position + self.ch.len_utf8(), self.src.len());

        if self.ch == '\n' {
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
            Token::new(TokenType::EOF, "".to_string()),
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(token.token_type, expected_token.token_type);
            assert_eq!(token.literal, expected_token.literal);
        }
    }
}
