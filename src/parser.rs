use crate::{
    Block, Inline, Lexer, Node, Program, Token, TokenType,
    block_quote::BlockQuote,
    code::{CodeBlock, InlineCode},
    expression::{Expression, VariableAccessExpression},
    heading::Heading,
    image::Image,
    inline_container::InlineContainer,
    link::Link,
    list::{ListItem, OrderedList, UnorderedList},
    marcblocks::{ForBlock, IfBlock, MarcBlock},
    text::{BoldText, ItalicizedText, ParagraphText, Text},
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

    fn advance_token_by_word(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_word());
    }

    fn parse(&mut self, end_token: TokenType, parse_inline: bool) -> Option<Box<dyn Node>> {
        let mut inline_container = InlineContainer::new();

        while let Some(token) = self.curr_token.clone() {
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
                TokenType::UnorderedListItem => {
                    self.advance_token();
                    let mut list = Box::new(UnorderedList::new());
                    self.parse_unordered_list_item(&mut list);
                    list
                }
                TokenType::OrderedListItem => {
                    self.advance_token();
                    let mut list = Box::new(OrderedList::new());
                    self.parse_ordered_list_item(&mut list);
                    list
                }
                TokenType::NewLine => Box::new(Text::new(token.literal)),
                TokenType::LeftDoubleBrace => self.parse_expression(),
                TokenType::RightDoubleBrace => todo!(),
                TokenType::KeywordStart => self.parse_keyword_block(),
                TokenType::KeywordEnd => self.parse_text(),
                TokenType::If => todo!(),
                TokenType::EndIf => todo!(),
                TokenType::For => todo!(),
                TokenType::EndFor => todo!(),
                TokenType::In => todo!(),
                TokenType::Import => todo!(),
                TokenType::Include => todo!(),
                TokenType::EOF => break,
                TokenType::True => todo!(),
                TokenType::False => todo!(),
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

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token.clone().unwrap().token_type == token_type {
            self.advance_token();
            true
        } else {
            false
        }
    }

    fn parse_keyword_block(&mut self) -> Box<MarcBlock> {
        println!("parsing keyword block");
        self.advance_token_by_word();

        let curr_token = self.curr_token.clone().unwrap();
        println!("curr_token: {:?}", curr_token);
        match curr_token.token_type {
            TokenType::For => {
                let for_block = self.parse_for_block();
                Box::new(MarcBlock::For(for_block))
            }
            TokenType::If => {
                let if_block = self.parse_if_block();
                Box::new(MarcBlock::If(if_block))
            }
            _ => {
                panic!("invalid token")
            }
        }
    }

    fn parse_if_block(&mut self) -> IfBlock {
        println!("parsing if block");
        println!("{:?}", self.curr_token);
        IfBlock::new(Expression::Empty)
    }

    fn parse_expression(&mut self) -> Box<Expression> {
        self.advance_token();
        self.advance_token();
        Box::new(Expression::Empty)
    }

    fn expect_next_token_or(&self, token: TokenType) -> bool {
        false
    }

    fn parse_text(&mut self) -> Box<dyn Node> {
        println!("parsing text...");
        let token = self.curr_token.clone().unwrap();
        println!("{token:#?}");

        if token.start_col > 1 {
            return Box::new(Text::new(token.literal));
        }

        let mut inline_container = Box::new(InlineContainer::new());
        inline_container.add_child(Box::new(Text::new(token.literal)));

        self.advance_token();
        let mut paragraph_text = ParagraphText::new();

        if self.curr_token.clone().unwrap().token_type == TokenType::NewLine {
            paragraph_text.set_inner(inline_container);
            return Box::new(paragraph_text);
        }

        let block = self.parse(TokenType::NewLine, true);
        if let Some(block) = block {
            match block.as_any().downcast::<InlineContainer>() {
                Ok(b) => {
                    inline_container.extend(*b);
                }
                Err(_) => {}
            }
            paragraph_text.set_inner(inline_container);
            println!("after parsing paragraph text {:#?}", self.curr_token);
        }

        self.advance_token();
        Box::new(paragraph_text)
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

    fn parse_italics(&mut self) -> Box<dyn Node> {
        println!("parsing italicized text");
        let mut italicized_text = ItalicizedText::new();
        self.advance_token();

        let block = self.parse(TokenType::Asterisk, true);
        if let Some(block) = block {
            italicized_text.set_inner(block);
            println!("after parsing italicized text {:#?}", self.curr_token);
        }

        self.advance_token();
        Box::new(italicized_text)
    }

    fn parse_blockquote(&mut self) -> Box<dyn Node> {
        self.advance_token();

        let curr_token = self.curr_token.clone().unwrap();
        if curr_token.token_type == TokenType::NewLine {
            return Box::new(Text::new(">".to_string()));
        }

        let mut block_quote = BlockQuote::new();

        let content = self.parse(TokenType::NewLine, true);
        if let Some(content) = content {
            println!("done parsing block quote");
            block_quote.set_inner(content);
        }

        self.advance_token();
        Box::new(block_quote)
    }

    fn parse_inline_code(&mut self) -> Box<dyn Node> {
        println!("parsing inline code");
        let mut code_content = String::new();

        self.advance_token();

        while let Some(token) = self.curr_token.clone() {
            if token.token_type == TokenType::NewLine || token.token_type == TokenType::EOF {
                return Box::new(Text::new(code_content));
            } else if token.token_type == TokenType::Backtick {
                return Box::new(InlineCode::new(code_content));
            }

            code_content.push_str(&token.literal);
            self.advance_token();
        }

        // this should never run
        Box::new(Text::new("".to_string()))
    }

    fn parse_code_block(&mut self) -> Box<dyn Node> {
        println!("parsing code block");

        self.advance_token();
        let language: String;
        let mut content = String::new();

        let mut curr_token = self.curr_token.clone().unwrap();

        match curr_token.token_type {
            TokenType::Text => {
                language = curr_token.literal.trim().to_string();
                self.advance_token(); // move past the text token (the language of the program)
                self.advance_token(); // move past the new line character token
            }
            TokenType::NewLine => {
                self.advance_token();
                language = String::new();
            }
            _ => return Box::new(Text::new("```".to_owned())),
        };

        curr_token = self.curr_token.clone().unwrap();

        while curr_token.token_type != TokenType::EOF
            && curr_token.token_type != TokenType::TripleBacktick
        {
            content.push_str(&curr_token.literal);
            self.advance_token();
            curr_token = self.curr_token.clone().unwrap();
        }

        self.advance_token(); // move past the code block so you can parse other tokens
        println!("{:?}", self.curr_token.to_owned().unwrap());

        Box::new(CodeBlock::new(content, language))
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

    fn parse_unordered_list_item(&mut self, list: &mut Box<UnorderedList>) {
        loop {
            let content = self.parse(TokenType::NewLine, true); // parse everything up until the
            // next new line character
            let mut list_item = Box::new(ListItem::new()); // create a list item

            if let Some(inner) = content {
                list_item.set_inner(inner); // set the inner of the list item to this parsed node
                list.add_list_item(list_item);
            } else {
                // content is only None when the end_token equals curr_token. so for this to
                // panic that means curr_token should equal the new line character but this
                // would be true because there is at least a white space (" ") character after
                // the - token for this function to even be called
                panic!(
                    "content should never return none because we'd definitely have a space text token after the - token"
                )
            }
            println!("curr token {:?}", self.curr_token.clone().unwrap());

            // at this point, curr_token should be the new line character;
            self.advance_token();
            self.advance_token(); // after this we should expect the next token to be an
            // unordered list item token. if it's not we can break the
            // loop and return the list node.
            //
            println!("curr token {:?}", self.curr_token.clone().unwrap());
            if self.curr_token.clone().unwrap().token_type == TokenType::UnorderedListItem {
                self.advance_token();
            } else {
                break;
            }
        }
    }

    fn parse_ordered_list_item(&mut self, list: &mut Box<OrderedList>) {
        println!("parsing ordered list....");
        loop {
            let content = self.parse(TokenType::NewLine, true); // parse everything up until the
            // next new line character
            let mut list_item = Box::new(ListItem::new()); // create a list item

            if let Some(inner) = content {
                list_item.set_inner(inner); // set the inner of the list item to this parsed node
                list.add_list_item(list_item);
            } else {
                // content is only None when the end_token equals curr_token. so for this to
                // panic that means curr_token should equal the new line character but this
                // would be true because there is at least a white space (" ") character after
                // the - token for this function to even be called
                panic!(
                    "content should never return none because we'd definitely have a space text token after the - token"
                )
            }
            println!("curr token {:?}", self.curr_token.clone().unwrap());

            // at this point, curr_token should be the new line character;
            self.advance_token();
            self.advance_token(); // after this we should expect the next token to be an
            // unordered list item token. if it's not we can break the
            // loop and return the list node.
            //
            println!("curr token {:?}", self.curr_token.clone().unwrap());
            if self.curr_token.clone().unwrap().token_type == TokenType::OrderedListItem {
                self.advance_token();
            } else {
                break;
            }
        }
    }

    fn parse_for_block(&self) -> ForBlock {
        ForBlock::new(
            VariableAccessExpression::new("".to_string()),
            VariableAccessExpression::new("".to_string()),
        )
    }
}
