use std::collections::HashMap;

use crate::expander::{
    ast::{
        Document, MarcNode,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        text_node::TextNode,
    },
    lexer::Lexer,
    parselets::{NameParselet, Parselets, PrefixParselet},
    token::{Token, TokenType as TT},
};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
    parselets: HashMap<TT, Parselets>,
}

impl Parser {
    pub fn register(&mut self, tt: TT, parselet: Parselets) {
        self.parselets.insert(tt, parselet);
    }

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
            parselets: HashMap::new(),
        };
        parser.advance_token();
        parser.advance_token();

        parser.register(TT::Minus, Parselets::Prefix(PrefixParselet::new()));
        parser.register(TT::Plus, Parselets::Prefix(PrefixParselet::new()));
        parser.register(TT::Exclamation, Parselets::Prefix(PrefixParselet::new()));
        parser.register(TT::Identifier, Parselets::Name(NameParselet::new()));
        parser
    }

    pub fn advance_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Box<MarcNode> {
        let marcnode = match self.curr_token.token_type {
            TT::Text => MarcNode::Text(TextNode::new(self.curr_token.literal.clone())),
            TT::KeywordStart => {
                let next_token = self.next_token.clone();
                match next_token.token_type {
                    TT::For => {
                        let for_block = self.parse_for_block();
                        MarcNode::For(for_block)
                    }
                    _ => {
                        let for_block = self.parse_for_block();
                        MarcNode::For(for_block)
                    }
                }
            }
            _ => MarcNode::Expression(Expression::Empty),
        };
        self.advance_token();
        Box::new(marcnode)
    }

    pub fn parse_expression(&mut self) -> Box<Expression> {
        let Some(parselet) = self.parselets.get(&self.curr_token.token_type) else {
            panic!("failed to parse expression. got {:?}", self.curr_token);
        };
        Box::new(parselet.parse, self.curr_token.clone()))
    }

    fn parse_for_block(&mut self) -> ForBlock {
        ForBlock::new(
            Expression::VariableAccess(VariableAccessExpression::new("products".to_string())),
            VariableAccessExpression::new("product".to_string()),
        )
    }

    pub fn parse_document(&mut self) -> Document {
        let mut program = Document::new();

        while self.curr_token.token_type != TT::EOF {
            let node = self.parse();
            program.add_node(node);
            self.advance_token();
        }

        program
    }
}
