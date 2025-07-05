use crate::{
    Block, Inline, Lexer, Node, Program, Token, TokenType,
    heading::Heading,
    image::Image,
    inline_container::InlineContainer,
    link::Link,
    text::{BoldText, Text},
};

pub struct Parser {
    curr_token: Option<Token>,
    peek_token: Option<Token>,
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            curr_token: None,
            peek_token: None,
            lexer: lexer,
        };

        // we're calling this twice so it can set both curr_token and next_token
        p.advance_token();
        p.advance_token();

        p
    }

    fn advance_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn parse(&mut self, end_token: TokenType, parse_inline: bool) -> Option<Box<dyn Node>> {
        let mut inline_container = InlineContainer::new();

        while let Some(token) = self.curr_token.clone() {
            println!("{:#?}", token);
            if token.token_type == end_token {
                return None;
            }
            let block: Box<dyn Node> = match token.token_type {
                TokenType::H1 => self.parse_heading(1),
                TokenType::H2 => self.parse_heading(2),
                TokenType::H3 => self.parse_heading(3),
                TokenType::H4 => self.parse_heading(4),
                TokenType::H5 => self.parse_heading(5),
                TokenType::H6 => self.parse_heading(6),
                TokenType::Asterisk => self.parse_italics(),
                TokenType::DoubleAsterisk => self.parse_bold_text(),
                TokenType::GreaterThan => self.parse_blockquote(),
                TokenType::Backtick => self.parse_inline_code(),
                TokenType::TripleBacktick => self.parse_code_block(),
                TokenType::LeftBracket => self.parse_link_start(),
                TokenType::RightBracket => self.parse_text(),
                TokenType::LeftParen => self.parse_text(),
                TokenType::RightParen => self.parse_text(),
                TokenType::Exclamation => self.parse_image(),
                TokenType::Text => self.parse_text(),
                TokenType::NewLine => Box::new(Text::new(token.literal)),
                TokenType::EOF => break,
            };
            println!("done parsing {}", block.token_literal());

            if parse_inline {
                inline_container.add_child(block);

                if self.peek_token.clone().unwrap().token_type == end_token {
                    break;
                }
                self.advance_token();
            } else {
                return Some(block);
            }
        }

        if parse_inline {
            Some(Box::new(inline_container))
        } else {
            None
        }
    }

    fn parse_text(&mut self) -> Box<dyn Node> {
        let token = self.curr_token.clone().unwrap();
        Box::new(Text::new(token.literal))
    }

    fn parse_link(&mut self) -> Result<(String, String), String> {
        let mut text = String::new(); // this is to keep track of the token literal seen so far so
        // we can return textual version if we encounter an
        // unexpected token before successfully parsing our image

        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be the '[' token
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::LeftBracket {
            return Err(text);
        }

        self.advance_token(); // update curr_token
        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be a text token
        // containing the alt description of the image
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::Text {
            return Err(text);
        }

        let link_text = curr_token.literal;

        self.advance_token();
        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be the ']' token
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::RightBracket {
            return Err(text);
        }

        self.advance_token();
        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be the '(' token
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::LeftParen {
            return Err(text);
        }

        self.advance_token();
        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be a text token
        // containing the src of the image
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::Text {
            return Err(text);
        }

        let image_src = curr_token.literal;

        self.advance_token();
        let curr_token = self.curr_token.clone().unwrap(); // this is expected to be a text token
        // containing the src of the image
        text.push_str(&curr_token.literal);

        if curr_token.token_type != TokenType::RightParen {
            return Err(text);
        }

        Ok((link_text, image_src))
    }

    fn parse_image(&mut self) -> Box<dyn Node> {
        self.advance_token(); // update curr_token
        let res = self.parse_link();
        if res.is_ok() {
            let content = res.unwrap();
            Box::new(Image::new(content.1, content.0))
        } else {
            Box::new(Text::new("!".to_string() + &res.err().unwrap()))
        }
    }

    pub fn parse_program(&mut self) -> Program {
        println!("parsing program");
        let mut program = Program::new();
        if self.curr_token.is_none() {
            return program;
        }

        while let Some(token) = self.curr_token.clone() {
            if token.token_type == TokenType::EOF {
                break;
            }

            let block = self.parse(TokenType::NewLine, false);
            if let Some(block) = block {
                println!("adding a new block to program {:?}", block.token_literal());
                program.add_block(block);
            }
            self.advance_token();
        }

        program
    }

    fn parse_bold_text(&mut self) -> Box<dyn Node> {
        println!("parsing bold text");
        let mut bold_text = BoldText::new();
        self.advance_token();

        let block = self.parse(TokenType::DoubleAsterisk, true);
        if let Some(block) = block {
            bold_text.set_inner(block);
            println!("after parsing bold text {:#?}", self.curr_token);
        }

        self.advance_token();
        Box::new(bold_text)
    }

    fn parse_heading(&mut self, level: i8) -> Box<dyn Block> {
        println!("parsing heaeding {}", level);
        self.advance_token();
        let mut h1 = Heading::new(level);

        let block = self.parse(TokenType::NewLine, true);
        if let Some(block) = block {
            h1.set_inner(block);
            println!("after parsing bold text {:#?}", self.curr_token);
        }

        self.advance_token();
        Box::new(h1)
    }

    fn parse_italics(&self) -> Box<dyn Block + 'static> {
        todo!()
    }

    fn parse_blockquote(&self) -> Box<dyn Block + 'static> {
        todo!()
    }

    fn parse_inline_code(&self) -> Box<dyn Block + 'static> {
        todo!()
    }

    fn parse_code_block(&self) -> Box<dyn Block + 'static> {
        todo!()
    }

    fn parse_link_start(&mut self) -> Box<dyn Inline> {
        print!("parsing a new link...");
        print!("with token {:#?}", self.curr_token);
        let res = self.parse_link();
        println!("{:?}", res);
        if res.is_ok() {
            let content = res.unwrap();
            Box::new(Link::new(content.0, content.1))
        } else {
            Box::new(Text::new("!".to_string() + &res.err().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Node,
        ast::heading::Heading,
        heading,
        text::{BoldText, Text},
    };

    use super::*;

    #[test]
    fn parse_image() {
        let input = "![this is the alt](https://google.com)";
        let lexer = Lexer::from(input);
        let mut parser = Parser::new(lexer);

        let image = Box::new(Image::new(
            "https://google.com".to_string(),
            "this is the alt".to_string(),
        ));

        let mut expected_program = Program::new();
        expected_program.add_block(image);

        let program = parser.parse_program();

        println!(
            "Got: {}, Expected: {}",
            program.token_literal(),
            expected_program.token_literal()
        );

        assert_eq!(program.token_literal(), expected_program.token_literal())
    }

    #[test]
    fn parse_link() {
        let input = "\
[this is the alt](https://google.com)
[hello world](https://jeremiah.vercel.app)";
        let lexer = Lexer::from(input);
        let mut parser = Parser::new(lexer);

        let link1 = Box::new(Link::new(
            "this is the alt".to_string(),
            "https://google.com".to_string(),
        ));

        let link2 = Box::new(Link::new(
            "hello world".to_string(),
            "https://jeremiah.vercel.app".to_string(),
        ));

        let mut expected_program = Program::new();
        expected_program.add_block(link1);
        expected_program.add_block(link2);

        let program = parser.parse_program();

        println!(
            "Got: {} \n Expected: {}",
            program.token_literal(),
            expected_program.token_literal()
        );

        assert_eq!(program.token_literal(), expected_program.token_literal())
    }

    #[test]
    fn parse_heading_1() {
        let input = "# Hello World\n## Hello World 2";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);

        let mut expected_program = Program::new();
        let mut heading_block = Box::new(Heading::new(1));
        let mut heading_2_block = Box::new(Heading::new(2));

        let mut inner_1 = Box::new(InlineContainer::new());
        let mut inner_2 = Box::new(InlineContainer::new());

        inner_1.add_child(Box::new(Text::new(" Hello World".to_string())));
        inner_2.add_child(Box::new(Text::new(" Hello World 2".to_string())));

        heading_block.set_inner(inner_1);
        heading_2_block.set_inner(inner_2);

        expected_program.add_block(heading_block);
        expected_program.add_block(heading_2_block);

        let parsed_program = p.parse_program();

        println!("Got: {}", parsed_program.token_literal());
        println!("Expected: {}", expected_program.token_literal());

        assert_eq!(
            parsed_program.token_literal(),
            expected_program.token_literal()
        );
    }

    #[test]
    fn parses_heading_with_inline_elements() {
        let input = "# Hello **World**";
        let lexer = Lexer::from(input);
        let mut p = Parser::new(lexer);

        let mut expected_program = Program::new();

        let mut heading_block = Box::new(Heading::new(1));
        let mut inner = Box::new(InlineContainer::new());
        inner.add_child(Box::new(Text::new(" Hello ".to_string())));

        let mut bold_text = Box::new(BoldText::new());
        let mut bold_inner = Box::new(InlineContainer::new());
        bold_inner.add_child(Box::new(Text::new("World".to_string())));
        bold_text.set_inner(bold_inner);

        inner.add_child(bold_text);

        heading_block.set_inner(inner);

        expected_program.add_block(heading_block);
        let parsed_program = p.parse_program();

        println!("{}", parsed_program.token_literal());
        println!("{}", expected_program.token_literal());

        assert_eq!(
            parsed_program.token_literal(),
            expected_program.token_literal()
        );
    }
}
