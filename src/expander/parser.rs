use crate::expander::{
    ast::{Document, MarcNode, text_node::TextNode},
    lexer::Lexer,
    token::{Token, TokenType as TT},
};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
}

impl Parser {
    pub fn from(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
        };
        parser.advance_token();
        parser.advance_token();
        parser
    }

    fn advance_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Box<MarcNode> {
        Box::new(MarcNode::Text(TextNode::new("Hello World!".to_string())))
    }

    pub fn parse_document(&mut self) -> Document {
        println!("parsing program");
        let mut program = Document::new();

        while self.curr_token.token_type != TT::EOF {
            let node = self.parse();
            program.add_block(node);
            self.advance_token();
        }

        program
    }
}
