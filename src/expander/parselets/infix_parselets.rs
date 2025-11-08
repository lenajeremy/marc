use crate::expander::{
    ast::{
        expression::{Expression, PrefixExpression, VariableAccessExpression},
        operators::Op,
    },
    parser::Parser,
    token::{Token, TokenType},
};

pub type InfixParseletFn =
    fn(parser: &mut Parser, left: Box<Expression>, token: Token) -> Box<Expression>;

pub fn parse_operator_infix(parser: &mut Parser, token: Token) -> Box<Expression> {
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

pub fn parse_variable_expression(_: &mut Parser, token: Token) -> Box<Expression> {
    if token.token_type == TokenType::Identifier {
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            token.literal,
        )))
    } else {
        panic!("expected an identifier token, got {}", token.literal);
    }
}
