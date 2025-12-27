use crate::expander::{
    ast::{
        expression::{Expression, InfixExpression},
        operators::Op,
    },
    parser::Parser,
};

pub type InfixParseletFn = fn(parser: &mut Parser, left: Box<Expression>) -> Box<Expression>;

pub fn parse_operator_infix(parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
    let token = parser.get_curr_token();

    let operator = Op::from_token(&token)
        .expect(format!("expected a valid operator token, got {}", token.literal).as_str());

    parser.advance_token();

    let next_expression = parser.parse_expression();

    Box::new(Expression::OperatorInfix(InfixExpression::new(
        left,
        next_expression,
        operator,
    )))
}
