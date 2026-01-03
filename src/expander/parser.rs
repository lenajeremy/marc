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
        InfixParselet, PrefixParselet, array_parselets::ArrayParselet,
        bracket_expression_parselets::GroupedExpressionParselet,
        function_call_parselet::FunctionCallParselet, integer_parselet::IntegerParselet,
        object_parselets::ObjectNotationParselet, operator_infix_parselets::OperatorInfixParselet,
        operator_prefix_parselets::OperatorPrefixParselet,
        variable_access_parselet::VariableAccessParselet,
    },
    precedence::Precendence,
    token::{Token, TokenType as TT},
};

pub struct Parser {
    curr_token: Token,
    next_token: Token,
    lexer: Lexer,
    prefix_parselets: HashMap<TT, &'static dyn PrefixParselet>,
    infix_parselets: HashMap<TT, &'static dyn InfixParselet>,
}

static VARIABLE_ACCESS_PARSELET: VariableAccessParselet = VariableAccessParselet;
static OPERATOR_PREFIX_PARSELET: OperatorPrefixParselet = OperatorPrefixParselet;
static INTEGER_PARSELET: IntegerParselet = IntegerParselet;
static GROUPED_OPERATION_PARSELET: GroupedExpressionParselet = GroupedExpressionParselet;
static OPERATOR_INFIX_PARSELET: OperatorInfixParselet = OperatorInfixParselet;
static FUNCTION_CALL_PARSELET: FunctionCallParselet = FunctionCallParselet;
static ARRAY_PARSELET: ArrayParselet = ArrayParselet;
static OBJECT_NOTATION_PARSELET: ObjectNotationParselet = ObjectNotationParselet;

impl Parser {
    fn register_prefix_parselet(&mut self, tt: TT, parselet: &'static dyn PrefixParselet) {
        self.prefix_parselets.insert(tt, parselet);
    }

    fn register_infix_parselet(&mut self, tt: TT, parselet: &'static dyn InfixParselet) {
        self.infix_parselets.insert(tt, parselet);
    }

    pub fn peek_token(&self) -> Token {
        self.next_token.clone()
    }

    pub fn get_curr_token(&self) -> Token {
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
        parser.register_prefix_parselet(TT::Identifier, &VARIABLE_ACCESS_PARSELET);
        parser.register_prefix_parselet(TT::Plus, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Minus, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Exclamation, &OPERATOR_PREFIX_PARSELET);
        parser.register_prefix_parselet(TT::Integer, &INTEGER_PARSELET);
        parser.register_prefix_parselet(TT::LeftParen, &GROUPED_OPERATION_PARSELET);

        // register infix parselets
        parser.register_infix_parselet(TT::Plus, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::Minus, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::ForwardSlash, &OPERATOR_INFIX_PARSELET);
        parser.register_infix_parselet(TT::Asterisk, &OPERATOR_INFIX_PARSELET);

        parser.register_infix_parselet(TT::Dot, &OBJECT_NOTATION_PARSELET);
        parser.register_infix_parselet(TT::LeftBracket, &ARRAY_PARSELET);
        parser.register_infix_parselet(TT::LeftParen, &FUNCTION_CALL_PARSELET);

        return parser;
    }

    pub fn advance_token(&mut self) {
        self.curr_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn parse(&mut self) -> Box<MarcNode> {
        let marcnode = match self.curr_token.token_type {
            TT::Text | TT::NewLine => {
                MarcNode::Text(TextNode::new(self.curr_token.literal.clone()))
            }
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
                MarcNode::Expression(self.parse_expression(0))
            }
            _ => MarcNode::Expression(Box::new(Expression::Empty)),
        };
        self.advance_token();
        Box::new(marcnode)
    }

    pub fn get_precedence(&self) -> u8 {
        println!(
            "\n--------\nfinding precedence of token: {:?}",
            self.curr_token.token_type
        );

        let infix_parselet = self.infix_parselets.get(&self.curr_token.token_type);
        let prefix_parselet = self.prefix_parselets.get(&self.curr_token.token_type);

        if infix_parselet.is_some() {
            return infix_parselet
                .unwrap()
                .get_precedence(self.curr_token.clone());
        }

        let token = match prefix_parselet {
            Some(x) => x.get_precedence(),
            None => 0,
        };

        println!(
            "precedence of token {:?} is {}\n--------",
            self.curr_token.token_type, token
        );

        token
    }

    pub fn parse_expression(&mut self, precendence: u8) -> Box<Expression> {
        println!(
            "parsing expression, curr_token: {:?}, previous_precedence: {}, curr_precedence: {}",
            self.curr_token,
            precendence,
            self.get_precedence(),
        );

        let cursor_details = self.lexer.get_cursor();
        let prefix_parselet = *self
            .prefix_parselets
            .get(&self.curr_token.token_type)
            .unwrap_or_else(|| {
                panic!(
                    "failed to parse expression. got {:?}, line: {}, column: {}",
                    self.curr_token, cursor_details.0, cursor_details.1
                );
            });

        let mut left = prefix_parselet.parse_expression(self, self.curr_token.clone());

        println!(
            "done parsing left, returned: {:?}, curr_token: {:?}, new_precedence {}",
            left.token_literal(),
            self.curr_token,
            self.get_precedence()
        );

        while precendence < self.get_precedence() {
            self.advance_token();
            println!("{:?}", self.curr_token);

            let infix_parselet = self
                .infix_parselets
                .get(&self.curr_token.token_type)
                .clone();

            left = match infix_parselet {
                Some(parselet) => {
                    let right = parselet.parse_expression(self, left);
                    println!("done parsing right, right is {:?}", right.token_literal());
                    right
                }
                _ => left,
            }
        }

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
            if node.token_literal() != Expression::Empty.token_literal() {
                program.add_node(node);
            }
            // self.advance_token();
        }

        program
    }
}
