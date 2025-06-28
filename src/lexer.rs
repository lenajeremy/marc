use crate::token::*;
use std::cmp::min;

struct Lexer {
    src: String,
    position: usize,
    read_position: usize,
    line: usize,
    col: usize,
    ch: u8,
}

impl Lexer {
    pub fn from(input: &str) -> Self {
        let mut lexer = Lexer {
            src: input.to_owned(),
            position: 0,
            read_position: 0,
            line: 0,
            col: 0,
            ch: 0,
        };

        lexer.read_char();
        lexer
    }

    pub fn eat_whitespace(&mut self) {
        while self.ch == b' ' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.ch {
            b'#' => {
                let mut total_pounds = 1;
                println!("The ASCII value of # is {}", b'#');
                println!("The peak_char is {}", self.peek_char(0));

                while self.peek_char(total_pounds) == b'#' && total_pounds < 6 {
                    total_pounds += 1;
                }

                match total_pounds {
                    1 => Token::new(TokenType::H1, "#".repeat(total_pounds + 1)),
                    2 => Token::new(TokenType::H2, "#".repeat(total_pounds + 1)),
                    3 => Token::new(TokenType::H3, "#".repeat(total_pounds + 1)),
                    4 => Token::new(TokenType::H4, "#".repeat(total_pounds + 1)),
                    5 => Token::new(TokenType::H5, "#".repeat(total_pounds + 1)),
                    6 => Token::new(TokenType::H6, "#".repeat(total_pounds + 1)),
                    _ => {
                        // this should never happen so long the expected condition is that
                        // total_pounds <= 6 is valid (which our while loop ensure) and so the ideal thing to do here is to
                        panic!(
                            "Total pounds = {}. Should never be greater than 6",
                            total_pounds
                        )
                    }
                }
            }
            b'\n' => Token::new(TokenType::NewLine, "\n".to_string()),
            _ => Token::new(TokenType::Invalid, "INVALID".to_string()),
        }
    }

    fn peek_char(&self, distance: usize) -> u8 {
        println!("Read Position: {}", self.read_position);
        let char_index = self.read_position + distance - 1;
        if char_index >= self.src.len() {
            println!("0");
            0
        } else {
            let byte = self.src[(char_index + distance)..].bytes().next().unwrap();
            println!("{}", byte);
            byte
        }
    }

    pub fn read_char(&mut self) {
        self.ch = match self.src[self.position..].bytes().next() {
            Some(x) => x,
            None => 0,
        };
        self.position = self.read_position;
        self.read_position = min(self.read_position + 1, self.src.len());
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_heading_1() {
        let input = "\
        # Hello World
        # This is Jeremiah
        # And this is a very important heading
";
        let mut lexer = Lexer::from(input);

        let expected_tokens = vec![
            Token::new(TokenType::NewLine, "\n".to_string()),
            Token::new(TokenType::H1, "#".to_string()),
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
