use crate::expander::{
    ast::expression::Expression,
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precedence,
    token::{Token, TokenType},
};

pub struct BooleanParselet;

impl PrefixParselet for BooleanParselet {
    fn get_precedence(&self) -> u8 {
        Precedence::VARIABLE as u8
    }

    fn parse_expression(&self, _: &mut Parser, token: Token) -> Box<Expression> {

        if token.token_type == TokenType::False || token.token_type == TokenType::True {
            let boolean_value = match token.token_type {
                TokenType::True => Expression::True,
                _ => Expression::False,
            };

            Box::new(boolean_value)
        } else {
            panic!("expected an integer token, got {}", token.literal);
        }
    }
}
