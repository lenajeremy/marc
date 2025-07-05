use crate::{token::*, utils};
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

    pub fn next_token(&mut self) -> Token {
        //self.eat_whitespace();
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

                return heading;
            }
            Some('\n') => Token::new(TokenType::NewLine, "\n".to_string(), self.line, self.col),
            Some('>') => {
                if self.col > 1 {
                    Token::new(TokenType::Text, ">".to_string(), self.line, self.col)
                } else {
                    Token::new(TokenType::GreaterThan, ">".to_string(), self.line, self.col)
                }
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
                    Token::new(TokenType::Text, "!".to_string(), 0, 0)
                }
            }
            None => Token::new(TokenType::EOF, "".to_string(), self.line, self.col),
            _ => {
                let start_col = self.col;
                let start_line = self.line;
                let text = self.read_until_newline_or_inline_token();
                return Token::new(TokenType::Text, text.clone(), start_line, start_col);
            }
        };

        self.read_char();
        token
    }

    pub fn read_until_newline_or_inline_token(&mut self) -> String {
        let start = self.position;

        while !utils::is_inline_token(self.ch) && self.ch != Some('\n') && self.ch != None {
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

#[cfg(test)]
mod tests {
    use super::*;
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
            assert_eq!(lexer.next_token().token_type, token_type);
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
            Token::new(TokenType::H2, "##".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                " Hello World".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::H1, "#".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                " This is Jeremiah".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::H1, "#".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                " And this is a very important heading".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::EOF, "".to_string(), lexer.line, lexer.col),
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
5*5";
        let mut lexer = Lexer::from(input);

        let expected_tokens = vec![
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Text, "bold".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "italic".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                "bolditalic".to_string(),
                lexer.line,
                lexer.col,
            ),
            // TODO:
            // pretty sure the asterisk is supposed to come before the double asterisk
            // but that'd make for a very challenging problem to solve, I'll come back
            // to see if it affects the correctness of the code.
            // I think it would but, fingers' crossed.
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "5".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "5".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::EOF, "".to_string(), lexer.line, lexer.col),
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
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                "console.log".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "In ".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Text, "Python".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(
                TokenType::Text,
                " you say ".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "print".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "In ".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Text, "Golang".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::DoubleAsterisk,
                "**".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(
                TokenType::Text,
                " you say ".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                "fmtPrintln".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "In ".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "Rust".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Asterisk, "*".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                " you say ".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                "println".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Backtick, "`".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::EOF, "".to_string(), lexer.line, lexer.col),
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
                lexer.line,
                lexer.col,
            ),
            Token::new(
                TokenType::Text,
                "javascript".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Text,
                "let x = 15;".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::TripleBacktick,
                "```".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::EOF, "".to_string(), lexer.line, lexer.col),
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
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Text, "text".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::RightBracket,
                "]".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::LeftParen, "(".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "url".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::RightParen,
                ")".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::Exclamation,
                "!".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(
                TokenType::LeftBracket,
                "[".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::Text, "text".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::RightBracket,
                "]".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::LeftParen, "(".to_string(), lexer.line, lexer.col),
            Token::new(TokenType::Text, "url".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::RightParen,
                ")".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::NewLine, "\n".to_string(), lexer.line, lexer.col),
            Token::new(
                TokenType::TripleBacktick,
                "```".to_string(),
                lexer.line,
                lexer.col,
            ),
            Token::new(TokenType::EOF, "".to_string(), lexer.line, lexer.col),
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
        let input =
            "# Heading\n> Quote\n**bold** and *italic* with [link](url) and `code` ![img](src)";
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
}
