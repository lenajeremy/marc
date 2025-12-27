use std::collections::HashMap;

use crate::expander::{
    ast::{
        Document, MarcNode, Node,
        expression::{Expression, VariableAccessExpression},
        marcblocks::ForBlock,
        text_node::TextNode,
    },
    lexer::Lexer,
    parselets::{
        array_parcelets::parse_array_expression,
        object_parselets::parse_object_expression,
        operator_infix_parselets::{InfixParseletFn, parse_operator_infix},
        variable_access_parselet::parse_variable_expression,
    },
    token::{Token, TokenType as TT},
};

use crate::expander::parselets::prefix_parselets::{PrefixParseletFn, parse_operator_prefix};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
    prefix_parselets: HashMap<TT, PrefixParseletFn>,
    infix_parselets: HashMap<TT, InfixParseletFn>,
}

impl Parser {
    fn register_prefix_parselet(&mut self, tt: TT, parselet: PrefixParseletFn) {
        self.prefix_parselets.insert(tt, parselet);
    }

    fn register_infix_parselet(&mut self, tt: TT, parselet: InfixParseletFn) {
        self.infix_parselets.insert(tt, parselet);
    }

    pub fn peek_token(&self) -> Token {
        self.next_token.clone()
    }

    pub fn curr_token(&self) -> Token {
        self.curr_token.clone()
    }

    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            curr_token: Token::new(TT::EOF, String::new(), 0, 0),
            next_token: Token::new(TT::EOF, String::new(), 0, 0),
            lexer,
            prefix_parselets: HashMap::new(),
            infix_parselets: HashMap::new(),
        };

        parser.advance_token();
        parser.advance_token();

        // register prefix parselets
        parser.register_prefix_parselet(TT::Identifier, parse_variable_expression);
        parser.register_prefix_parselet(TT::Plus, parse_operator_prefix);
        parser.register_prefix_parselet(TT::Minus, parse_operator_prefix);
        parser.register_prefix_parselet(TT::Exclamation, parse_operator_prefix);

        // register infix parselets
        parser.register_infix_parselet(TT::Plus, parse_operator_infix);
        parser.register_infix_parselet(TT::Minus, parse_operator_infix);
        parser.register_infix_parselet(TT::ForwardSlash, parse_operator_infix);

        parser.register_infix_parselet(TT::Dot, parse_object_expression);
        parser.register_infix_parselet(TT::LeftBracket, parse_array_expression);

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
            TT::LeftDoubleBrace => {
                self.advance_token();
                MarcNode::Expression(self.parse_expression())
            }
            _ => MarcNode::Expression(Box::new(Expression::Empty)),
        };
        self.advance_token();
        Box::new(marcnode)
    }

    pub fn parse_expression(&mut self) -> Box<Expression> {
        println!("parsing expression, curr_token: {:?}", self.curr_token);
        let cursor_details = self.lexer.get_cursor();
        let prefix_parselet = self
            .prefix_parselets
            .get(&self.curr_token.token_type)
            .clone()
            .unwrap_or_else(|| {
                panic!(
                    "failed to parse expression. got {:?}, line: {}, column: {}",
                    self.curr_token, cursor_details.0, cursor_details.1
                );
            });

        let left = prefix_parselet(self, self.curr_token.clone());

        left
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
            // println!(
            //     "parsing node: {:?}
            //     curr_token: {:?}",
            //     node.token_literal(),
            //     self.curr_token()
            // );
            program.add_node(node);
            // self.advance_token();
        }

        program
    }
}
