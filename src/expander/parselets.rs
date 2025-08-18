use crate::expander::{
    ast::{
        expression::{Expression, PrefixExpression, VariableAccessExpression},
        operators::Op,
    },
    parser::Parser,
    token::{Token, TokenType},
};

pub trait Parselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression;
}

pub struct PrefixParselet;
impl PrefixParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parselet for PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Expression {
        let operator = Op::from_token(&token);
        if operator.is_none() {
            panic!("expected a valid operator token, got {}", token.literal);
        }

        let operator = operator.unwrap();
        parser.advance_token();
        let next_expression = parser.parse_expression();
        Expression::Prefix(PrefixExpression::new(operator, next_expression))
    }
}

pub struct NameParselet;
impl NameParselet {
    pub fn new() -> Self {
        Self {}
    }
}

impl Parselet for NameParselet {
    fn parse(&self, _: &mut Parser, token: Token) -> Expression {
        if token.token_type == TokenType::Identifier {
            Expression::VariableAccess(VariableAccessExpression::new(token.literal))
        } else {
            panic!("expected an identifier token, got {}", token.literal);
        }
    }
}

pub enum Parselets {
    Name(NameParselet),
    Prefix(PrefixParselet),
}

impl Parselets {
    pub fn parse(&self, parser: &mut Parser, token: Token) -> Expression {
        match self {
            Parselets::Name(name_parselet) => name_parselet.parse(parser, token),
            Parselets::Prefix(prefix_parselet) => prefix_parselet.parse(parser, token),
        }
    }
}
