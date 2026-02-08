use crate::expander::{
    ast::expression::{Expression, IntegerExpression},
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precedence,
    token::{Token, TokenType},
};

pub struct IntegerParselet;

impl PrefixParselet for IntegerParselet {
    fn get_precedence(&self) -> u8 {
        Precedence::VARIABLE as u8
    }

    fn parse_expression(&self, _: &mut Parser, token: Token) -> Box<Expression> {
        if token.token_type == TokenType::Integer {
            let integer_value = token.literal.parse::<i64>().expect("invalid integer value");

            Box::new(Expression::Integer(IntegerExpression::new(integer_value)))
        } else {
            panic!("expected an integer token, got {}", token.literal);
        }
    }
}
