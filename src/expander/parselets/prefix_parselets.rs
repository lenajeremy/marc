use crate::expander::{
    ast::{
        expression::{Expression, PrefixExpression},
        operators::Op,
    },
    parser::Parser,
    token::Token,
};

pub type PrefixParseletFn = fn(parser: &mut Parser, token: Token) -> Box<Expression>;

pub fn parse_operator_prefix(parser: &mut Parser, token: Token) -> Box<Expression> {
    let operator = Op::from_token(&token);
    if operator.is_none() {
        panic!("expected a valid operator token, got {}", token.literal);
    }

    let operator = operator.unwrap();
    parser.advance_token();
    let next_expression = parser.parse_expression();
    Box::new(Expression::Prefix(PrefixExpression::new(
        operator,
        next_expression,
    )))
}
