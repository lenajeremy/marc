use crate::expander::{
    ast::expression::{Expression, ObjectAccessExpression},
    parser::Parser,
};

pub fn parse_object_expression(parser: &mut Parser, parent: Box<Expression>) -> Box<Expression> {
    parser.advance_token(); // move the cursor past the `.` char to the next token.

    let next_expression = parser.parse_expression();

    Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
        parent,
        next_expression,
    )))
}
