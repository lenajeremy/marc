use crate::expander::{
    ast::expression::{ArrayAccessExpression, Expression},
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precedence,
    token::{Token, TokenType},
};

pub struct ArrayParselet;

impl InfixParselet for ArrayParselet {
    fn get_precedence(&self, _: Token) -> u8 {
        Precedence::ArrayAccess as u8
    }

    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
        parser.advance_token(); // move the cursor past the `[` char to the next token.

        let next_expression = parser.parse_expression(0);

        if parser.get_curr_token().token_type != TokenType::RightBracket {
            if parser.peek_token().token_type != TokenType::RightBracket {
                panic!("expected ']' after array index expression");
            }
            parser.advance_token();
        }

        Box::new(Expression::ArrayAccess(ArrayAccessExpression::new(
            left,
            next_expression,
        )))
    }
}
