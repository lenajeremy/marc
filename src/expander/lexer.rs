use std::cmp::min;

use crate::expander::token::{Token, TokenType as TT};
use crate::utils;

#[derive(Debug)]
pub struct Lexer {
    is_detailed: bool,
    pub src: String,
    position: usize,
    read_position: usize,
    line: usize,
    col: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn get_cursor(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn from(input: &str) -> Self {
        let input = Self::clean_input(input);
        let len = input.len();

        let mut lexer = Lexer {
            is_detailed: false,
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
        while self.ch == Some(' ') {
            self.read_char();
        }
    }

    pub fn next_word(&mut self) -> Token {
        self.eat_whitespace();
        let start_position = self.position;
        let start_line = self.line;
        let start_col = self.col;

        while utils::is_alphanumeric(self.ch) {
            self.read_char();
        }

        let text = self.src[start_position..self.position].to_string();
        let token = match text.as_str() {
            "if" => Token::new(TT::If, text, start_line, start_col),
            "endif" => Token::new(TT::EndIf, text, start_line, start_col),
            "for" => Token::new(TT::For, text, start_line, start_col),
            "endfor" => Token::new(TT::EndFor, text, start_line, start_col),
            "include" => Token::new(TT::Include, text, start_line, start_col),
            "import" => Token::new(TT::Import, text, start_line, start_col),
            "in" => Token::new(TT::In, text, start_line, start_col),
            "false" => Token::new(TT::False, text, start_line, start_col),
            "true" => Token::new(TT::True, text, start_line, start_col),
            "as" => Token::new(TT::As, text, start_line, start_col),
            _ => Token::new(TT::Identifier, text, start_line, start_col),
        };

        self.eat_whitespace();
        token
    }

    pub fn next_token(&mut self) -> Token {
        if self.ch.is_none() {
            return Token::new(TT::EOF, "".to_string(), self.line, self.col);
        }

        if self.is_detailed {
            self.eat_whitespace();
        } else {
            let start_position = self.position;
            let start_line = self.line;
            let start_col = self.col;

            if self.ch.unwrap() == '\n' {
                self.read_char();
                return Token::new(TT::NewLine, "\n".to_string(), start_line, start_col);
            }

            if self.ch.unwrap() == '@' {
                self.is_detailed = true;
                self.read_char();
                return Token::new(TT::At, "@".to_string(), start_line, start_col);
            }

            while let Some(ch) = self.ch {
                let next_char = self.peek_char(1);
                if ch == '\n' || (ch == '{' && (next_char == '%' || next_char == '{')) {
                    self.is_detailed = ch != '\n'; // if ch is the new line character,
                    // self.is_detailed should be false,
                    break;
                }
                self.read_char();
            }

            if self.position != start_position {
                let text = self.src[start_position..self.position].to_string();
                return Token::new(TT::Text, text, start_line, start_col);
            }
        }

        let token = match self.ch {
            Some('\n') => Token::new(TT::NewLine, "\n".to_string(), self.line, self.col),
            Some('\"') => Token::new(TT::DoubleQuote, "\"".to_string(), self.line, self.col),
            Some('\'') => Token::new(TT::SingleQuote, "'".to_string(), self.line, self.col),
            Some('>') => Token::new(TT::GreaterThan, ">".to_string(), self.line, self.col),
            Some('-') => Token::new(TT::Minus, "-".to_string(), self.line, self.col),
            Some('*') => Token::new(TT::Minus, "*".to_string(), self.line, self.col),
            Some('.') => Token::new(TT::Dot, ".".to_string(), self.line, self.col),
            Some('[') => Token::new(TT::LeftBracket, "[".to_string(), self.line, self.col),
            Some(']') => Token::new(TT::RightBracket, "]".to_string(), self.line, self.col),
            Some('(') => Token::new(TT::LeftParen, "(".to_string(), self.line, self.col),
            Some(')') => Token::new(TT::RightParen, ")".to_string(), self.line, self.col),
            Some('!') => Token::new(TT::Exclamation, "!".to_string(), self.line, self.col),
            Some(',') => Token::new(TT::Comma, ",".to_string(), self.line, self.col),
            Some(';') => {
                if self.peek_char(1) == '\n' {
                    self.is_detailed = false;
                }
                Token::new(TT::Semicolon, ";".to_string(), self.line, self.col)
            }
            Some('=') => {
                let peek_char = self.peek_char(1);
                if peek_char == '=' {
                    self.read_char();
                    Token::new(TT::Equals, "==".to_string(), self.line, self.col)
                } else {
                    Token::new(TT::Assign, "=".to_string(), self.line, self.col)
                }
            }
            Some('{') => {
                let start_line = self.line;
                let start_col = self.col;

                let token = match self.peek_char(1) {
                    '{' => {
                        self.read_char();
                        self.read_char();
                        Token::new(TT::LeftDoubleBrace, "{{".to_string(), start_line, start_col)
                    }
                    '%' => {
                        self.read_char();
                        self.read_char();
                        Token::new(TT::KeywordStart, "{%".to_string(), start_line, start_col)
                    }
                    _ => Token::new(TT::Text, "{".to_string(), start_line, start_col),
                };
                return token;
            }
            Some('%') => {
                let next_char = self.peek_char(1);
                let start_line = self.line;
                let start_col = self.col;

                if next_char == '}' {
                    self.read_char();
                    self.is_detailed = false;
                    Token::new(TT::KeywordEnd, "%}".to_string(), start_line, start_col)
                } else {
                    Token::new(TT::Text, "%".to_string(), start_line, start_col)
                }
            }
            Some('}') => {
                let next_char = self.peek_char(1);
                let start_line = self.line;
                let start_col = self.col;

                if next_char == '}' {
                    self.read_char();
                    self.is_detailed = false;
                    Token::new(
                        TT::RightDoubleBrace,
                        "}}".to_string(),
                        start_line,
                        start_col,
                    )
                } else {
                    Token::new(TT::Text, "}".to_string(), start_line, start_col)
                }
            }
            None => Token::new(TT::EOF, "".to_string(), self.line, self.col),
            _ => {
                if self.ch.unwrap().is_numeric() {
                    let start_line = self.line;
                    let start_col = self.col;
                    let integer = self.read_integer();
                    return Token::new(TT::Integer, integer, start_line, start_col);
                }
                return self.next_word();
            }
        };

        self.read_char();
        token
    }

    fn read_integer(&mut self) -> String {
        let start_position = self.position;
        while self.ch.unwrap().is_numeric() {
            self.read_char();
        }
        self.src[start_position..self.position].to_string()
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

    fn read_char(&mut self) {
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
