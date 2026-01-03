use crate::expander::{
    ast::{
        Node,
        expression::{ArrayAccessExpression, Expression},
    },
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precendence,
    token::Token,
};

pub struct ArrayParselet;

impl InfixParselet for ArrayParselet {
    fn get_precedence(&self, _: Token) -> u8 {
        Precendence::ARRAYACCESS as u8
    }

    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
        println!(
            "parsing an array, token is {:?} left is {:?}",
            parser.get_curr_token(),
            left.token_literal()
        );

        parser.advance_token(); // move the cursor past the `[` char to the next token.

        println!(
            "\nnext token: {:?}\nprecedence: {}\n",
            parser.get_curr_token().token_type,
            parser.get_precedence()
        );

        let next_expression = parser.parse_expression(self.get_precedence(parser.get_curr_token()));

        Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
            left,
            next_expression,
        )))
    }
}
