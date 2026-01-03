use crate::expander::{
    ast::expression::{Expression, ObjectAccessExpression},
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precendence,
    token::Token,
};

pub struct ObjectNotationParselet;

impl InfixParselet for ObjectNotationParselet {
    fn get_precedence(&self, _token: Token) -> u8 {
        Precendence::OBJECTACCESS as u8
    }

    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
        parser.advance_token(); // move the cursor past the `.` char to the next token.

        let next_expression = parser.parse_expression(self.get_precedence(parser.get_curr_token()));

        Box::new(Expression::ObjectAccess(ObjectAccessExpression::new(
            left,
            next_expression,
        )))
    }
}
