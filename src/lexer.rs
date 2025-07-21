use crate::{is_numeric, token::*, utils};
use std::cmp::min;

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

    pub fn get_line(&self) -> usize {
        self.line
    }

    pub fn get_col(&self) -> usize {
        self.col
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

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some('#') => {
                if self.col > 1 {
                    // if the # char is not the first
                    let t = Token::new(TokenType::Text, "#".to_string(), self.line, self.col);
                    self.read_char();
                    return t;
                }

                let mut total_pounds = 1;

                while self.peek_char(total_pounds) == '#' && total_pounds < 6 {
                    total_pounds += 1;
                }

                let heading = match total_pounds {
                    1 => Token::new(TokenType::H1, "#".repeat(total_pounds), self.line, self.col),
                    2 => Token::new(TokenType::H2, "#".repeat(total_pounds), self.line, self.col),
                    3 => Token::new(TokenType::H3, "#".repeat(total_pounds), self.line, self.col),
                    4 => Token::new(TokenType::H4, "#".repeat(total_pounds), self.line, self.col),
                    5 => Token::new(TokenType::H5, "#".repeat(total_pounds), self.line, self.col),
                    6 => Token::new(TokenType::H6, "#".repeat(total_pounds), self.line, self.col),
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

                if let Some(curr_char) = self.ch {
                    if curr_char == ' ' {
                        return heading;
                    }
                }

                // headers should be the first characters in the line
                return Token::new(TokenType::Text, "#".repeat(total_pounds), self.line, 1);
            }
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
                let token: Token;

                if self.peek_char(1) != ' ' {
                    token = Token::new(TokenType::Text, "-".to_string(), start_line, start_col);
                } else {
                    token = Token::new(
                        TokenType::UnorderedListItem,
                        "-".to_string(),
                        start_line,
                        start_col,
                    )
                }
                self.read_char();
                token
            }
            Some('*') => {
                let next_char = self.peek_char(1);

                let start_line = self.line;
                let start_col = self.col;

                if next_char == '*' {
                    self.read_char();
                    Token::new(
                        TokenType::DoubleAsterisk,
                        "**".to_string(),
                        start_line,
                        start_col,
                    )
                } else {
                    Token::new(TokenType::Asterisk, "*".to_string(), self.line, self.col)
                }
            }
            Some('`') => {
                let next = self.peek_char(1);
                if next == '`' && self.peek_char(2) == '`' {
                    let tok_type = if self.col == 1 {
                        TokenType::TripleBacktick
                    } else {
                        TokenType::Text
                    };

                    let tok = Token::new(tok_type, "```".to_string(), self.line, self.col);
                    self.read_char();
                    self.read_char();
                    tok
                } else {
                    Token::new(TokenType::Backtick, "`".to_string(), self.line, self.col)
                }
            }
            Some('[') => Token::new(TokenType::LeftBracket, "[".to_string(), self.line, self.col),
            Some(']') => Token::new(
                TokenType::RightBracket,
                "]".to_string(),
                self.line,
                self.col,
            ),
            Some('(') => Token::new(TokenType::LeftParen, "(".to_string(), self.line, self.col),
            Some(')') => Token::new(TokenType::RightParen, ")".to_string(), self.line, self.col),
            Some('!') => {
                let next_char = self.peek_char(1);
                if next_char == '[' {
                    Token::new(TokenType::Exclamation, "!".to_string(), self.line, self.col)
                } else {
                    Token::new(TokenType::Text, "!".to_string(), self.line, self.col)
                }
            }
            Some('{') => {
                let start_line = self.line;
                let start_col = self.col;

                let token = match self.peek_char(1) {
                    '{' => {
                        self.read_char();
                        Token::new(
                            TokenType::LeftDoubleBrace,
                            "{{".to_string(),
                            start_line,
                            start_col,
                        )
                    }
                    '%' => {
                        self.read_char(); // moves self.ch to equal %
                        Token::new(
                            TokenType::KeywordStart,
                            "{%".to_string(),
                            start_line,
                            start_col,
                        )
                    }
                    _ => Token::new(TokenType::Text, "{".to_string(), start_line, start_col),
                };
                token
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

                if is_numeric(self.ch) && self.peek_char(1) == '.' && self.peek_char(2) == ' ' {
                    let literal = format!("{}. ", self.ch.unwrap());
                    let token =
                        Token::new(TokenType::OrderedListItem, literal, start_line, start_col);
                    self.read_char();
                    self.read_char();
                    self.read_char();

                    return token;
                }

                let text = self.read_until_newline_or_inline_token();

                match text.as_str() {
                    "if" => Token::new(TokenType::If, text, start_line, start_col),
                    "endif" => Token::new(TokenType::EndIf, text, start_line, start_col),
                    "for" => Token::new(TokenType::For, text, start_line, start_col),
                    "endfor" => Token::new(TokenType::EndFor, text, start_line, start_col),
                    "include" => Token::new(TokenType::Include, text, start_line, start_col),
                    "import" => Token::new(TokenType::Import, text, start_line, start_col),
                    "in" => Token::new(TokenType::In, text, start_line, start_col),
                    _ => Token::new(TokenType::Text, text, start_line, start_col),
                }
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

    //fn prev_char(&self) -> char {
    //    println!("calling prev_char {self:?}");
    //    if self.position == 0 {
    //        '\0'
    //    } else {
    //        let prev = self.src[(self.position - 1)..].chars().next().unwrap();
    //        println!("prev is {}", prev);
    //        prev
    //    }
    //}

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
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}
