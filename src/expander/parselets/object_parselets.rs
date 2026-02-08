use crate::expander::{
    ast::expression::{Expression, ObjectAccessExpression},
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precedence,
    token::Token,
};

pub struct ObjectNotationParselet;

impl InfixParselet for ObjectNotationParselet {
    fn get_precedence(&self, _token: Token) -> u8 {
        Precedence::OBJECTACCESS as u8
    }

    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
        let precedence = Precedence::CALL as u8;
        parser.advance_token(); // move the cursor past the `.` char to the next token.

        let next_expression = parser.parse_expression(precedence);

        Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
            left,
            next_expression,
        )))
    }
}
