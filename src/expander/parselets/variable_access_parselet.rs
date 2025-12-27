use crate::expander::{
    ast::expression::{Expression, VariableAccessExpression},
    parser::Parser,
    token::{Token, TokenType},
};

pub type VariableAccessParseletFn = fn(parser: &mut Parser, token: Token) -> Box<Expression>;

pub fn parse_variable_expression(_: &mut Parser, token: Token) -> Box<Expression> {
    if token.token_type == TokenType::Identifier {
        Box::new(Expression::VariableAccess(VariableAccessExpression::new(
            token.literal,
        )))
    } else {
        panic!("expected an identifier token, got {}", token.literal);
    }
}
