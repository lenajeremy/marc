use crate::expander::{
    ast::expression::Expression, parselets::PrefixParselet, parser::Parser,
    precedence::Precendence, token::Token,
};

pub struct GroupedExpressionParselet;

impl PrefixParselet for GroupedExpressionParselet {
    fn get_precedence(&self) -> u8 {
        Precendence::PREFIX as u8
    }

    fn parse_expression(&self, parser: &mut Parser, _: Token) -> Box<Expression> {
        parser.advance_token(); // move the cursor past the `(` char to the next token.

        let next_expression = parser.parse_expression(self.get_precedence());

        next_expression
    }
}
