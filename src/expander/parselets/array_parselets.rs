use crate::expander::{
    ast::expression::{ArrayAccessExpression, Expression},
    parser::Parser,
};

pub fn parse_array_expression(parser: &mut Parser, array: Box<Expression>) -> Box<Expression> {
    parser.advance_token(); // move the cursor past the `[` char to the next token.

    let next_expression = parser.parse_expression();

    Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
        array,
        next_expression,
    )))
}
