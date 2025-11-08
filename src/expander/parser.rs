use std::collections::HashMap;

use crate::expander::{
    ast::{
        Document, MarcNode,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        text_node::TextNode,
    },
    lexer::Lexer,
    token::{Token, TokenType as TT},
};

use crate::expander::parselets::prefix_parselets::{
    PrefixParseletFn, parse_operator_prefix, parse_variable_expression,
};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
    prefix_parselets: HashMap<TT, PrefixParseletFn>,
}

impl Parser {
    fn register_prefix_parselet(&mut self, tt: TT, parselet: PrefixParseletFn) {
        self.prefix_parselets.insert(tt, parselet);
    }

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
            prefix_parselets: HashMap::new(),
        };

        parser.advance_token();
        parser.advance_token();

        parser.register_prefix_parselet(TT::Identifier, parse_variable_expression);
        parser.register_prefix_parselet(TT::Plus, parse_operator_prefix);
        parser.register_prefix_parselet(TT::Minus, parse_operator_prefix);
        parser.register_prefix_parselet(TT::Exclamation, parse_operator_prefix);

        return parser;
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
        let Some(parselet) = self.prefix_parselets.get(&self.curr_token.token_type) else {
            panic!("failed to parse expression. got {:?}", self.curr_token);
        };
        parselet(self, self.curr_token.clone())
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
