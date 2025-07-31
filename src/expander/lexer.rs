use std::cmp::min;

use crate::expander::token::{Token, TokenType};
use crate::utils;

#[derive(Debug)]
pub struct Lexer {
    pub src: String,
    position: usize,
    read_position: usize,
    line: usize,
    col: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn from(input: &str) -> Self {
        let input = Self::clean_input(input);
        let len = input.len();

        let mut lexer = Lexer {
            src: input,
            position: 0,
            read_position: 0,
            line: if len == 0 { 0 } else { 1 },
            col: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    /// `clean_input` removes only space characters from both ends of each line of the provided
    /// input.
    ///
    /// `clean_input` only removes space(' ') characters because other white space
    /// characters like \t or \n can be useful in parts of the input. eg, a \n character indicates
    /// the end of a new line and should not be removed.
    pub fn clean_input(input: &str) -> String {
        //let s: String = input
        //    .lines()
        //    .map(|line| line.trim_matches(' ').to_owned() + "\n")
        //    .collect();
        //s.trim_end().to_owned()
        input.to_owned()
    }

    pub fn eat_whitespace(&mut self) {
        println!("eating whitespace, \"{:?}\"", self.ch);
        while self.ch == Some(' ') {
            self.read_char();
        }
    }

    //pub fn next_word(&mut self) -> Token {
    //    self.eat_whitespace();
    //    let start_position = self.position;
    //    let start_line = self.line;
    //    let start_col = self.col;
    //
    //    while self.ch.unwrap() != ' ' {
    //        self.read_char();
    //    }
    //
    //    let text = self.src[start_position..self.position].to_string();
    //    let token = match text.as_str() {
    //        "if" => Token::new(TokenType::If, text, start_line, start_col),
    //        "endif" => Token::new(TokenType::EndIf, text, start_line, start_col),
    //        "for" => Token::new(TokenType::For, text, start_line, start_col),
    //        "endfor" => Token::new(TokenType::EndFor, text, start_line, start_col),
    //        "include" => Token::new(TokenType::Include, text, start_line, start_col),
    //        "import" => Token::new(TokenType::Import, text, start_line, start_col),
    //        "in" => Token::new(TokenType::In, text, start_line, start_col),
    //        _ => Token::new(TokenType::Text, text, start_line, start_col),
    //    };
    //
    //    self.eat_whitespace();
    //    token
    //}

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('\n') => Token::new(TokenType::NewLine, "\n".to_string(), self.line, self.col),
            Some('>') => {
                if self.col > 1 {
                    Token::new(TokenType::Text, ">".to_string(), self.line, self.col)
                } else {
                    Token::new(TokenType::GreaterThan, ">".to_string(), self.line, self.col)
                }
            }
            Some('-') => {
                let start_line = self.line;
                let start_col = self.col;
                let token = Token::new(TokenType::Minus, "-".to_string(), start_line, start_col);

                token
            }
            Some('*') => Token::new(TokenType::Asterisk, "*".to_string(), self.line, self.col),
            Some('[') => Token::new(TokenType::LeftBracket, "[".to_string(), self.line, self.col),
            Some(']') => Token::new(
                TokenType::RightBracket,
                "]".to_string(),
                self.line,
                self.col,
            ),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string(), self.line, self.col),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string(), self.line, self.col),
            Some('!') => Token::new(TokenType::Exclamation, "!".to_string(), self.line, self.col),
            Some('{') => {
                let start_line = self.line;
                let start_col = self.col;

                let token = match self.peek_char(1) {
                    '{' => {
                        self.read_char();
                        self.read_char();
                        Token::new(
                            TokenType::LeftDoubleBrace,
                            "{{".to_string(),
                            start_line,
                            start_col,
                        )
                    }
                    '%' => {
                        self.read_char();
                        self.read_char();
                        self.eat_whitespace();
                        Token::new(
                            TokenType::KeywordStart,
                            "{%".to_string(),
                            start_line,
                            start_col,
                        )
                    }
                    _ => Token::new(TokenType::Text, "{".to_string(), start_line, start_col),
                };
                return token;
            }
            Some('%') => {
                let next_char = self.peek_char(1);
                let start_line = self.line;
                let start_col = self.col;

                if next_char == '}' {
                    self.read_char();
                    Token::new(
                        TokenType::KeywordEnd,
                        "%}".to_string(),
                        start_line,
                        start_col,
                    )
                } else {
                    Token::new(TokenType::Text, "%".to_string(), start_line, start_col)
                }
            }
            Some('}') => {
                let next_char = self.peek_char(1);
                let start_line = self.line;
                let start_col = self.col;

                if next_char == '}' {
                    self.read_char();
                    Token::new(
                        TokenType::RightDoubleBrace,
                        "}}".to_string(),
                        start_line,
                        start_col,
                    )
                } else {
                    Token::new(TokenType::Text, "}".to_string(), start_line, start_col)
                }
            }
            None => Token::new(TokenType::EOF, "".to_string(), self.line, self.col),
            _ => {
                let start_col = self.col;
                let start_line = self.line;

                let text = self.read_until_newline_or_inline_token();
                let token = match text.as_str() {
                    "if" => Token::new(TokenType::If, text, start_line, start_col),
                    "endif" => Token::new(TokenType::EndIf, text, start_line, start_col),
                    "for" => Token::new(TokenType::For, text, start_line, start_col),
                    "endfor" => Token::new(TokenType::EndFor, text, start_line, start_col),
                    "include" => Token::new(TokenType::Include, text, start_line, start_col),
                    "import" => Token::new(TokenType::Import, text, start_line, start_col),
                    "in" => Token::new(TokenType::In, text, start_line, start_col),
                    "true" => Token::new(TokenType::True, text, start_line, start_col),
                    "false" => Token::new(TokenType::False, text, start_line, start_col),
                    _ => Token::new(TokenType::Text, text, start_line, start_col),
                };

                return token;
            }
        };

        self.read_char();
        token
    }

    pub fn read_until_newline_or_inline_token(&mut self) -> String {
        let start = self.position;

        while !utils::is_inline_token(self.ch) && self.ch != Some('\n') && self.ch != None {
            if self.ch.is_some() && self.ch.unwrap() == ' ' {
                let word = self.src[start..self.position].trim().to_string();
                if utils::is_keyword(&word) {
                    return word;
                }
            }
            self.read_char();
        }

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
        self.ch = self.src[self.read_position..].chars().next();
        self.position = self.read_position;
        if let Some(x) = self.ch {
            self.read_position = min(self.read_position + x.len_utf8(), self.src.len());
        } else {
            self.read_position = min(self.read_position, self.src.len());
        }

        if self.ch == Some('\n') {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}
