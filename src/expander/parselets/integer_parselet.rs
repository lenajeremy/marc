use crate::expander::{
    ast::expression::{Expression, IntegerExpression},
    parser::Parser,
    token::{Token, TokenType},
};

pub type IntegerParseletFn = fn(parser: &mut Parser, token: Token) -> Box<Expression>;

pub fn parse_integer_expression(_: &mut Parser, token: Token) -> Box<Expression> {
    if token.token_type == TokenType::Integer {
        let integer_value = token.literal.parse::<i64>().expect("invalid integer value");

        Box::new(Expression::Integer(IntegerExpression::new(integer_value)))
    } else {
        panic!("expected an integer token, got {}", token.literal);
    }
}
