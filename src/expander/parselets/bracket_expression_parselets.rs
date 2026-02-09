use crate::expander::{
    ast::expression::Expression,
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precedence,
    token::{Token, TokenType},
};

pub struct GroupedExpressionParselet;

impl PrefixParselet for GroupedExpressionParselet {
    fn get_precedence(&self) -> u8 {
        Precedence::PREFIX as u8
    }

    fn parse_expression(&self, parser: &mut Parser, _: Token) -> Box<Expression> {
        parser.advance_token(); // move the cursor past the `(` char to the next token.

        let next_expression = parser.parse_expression(0);

        if parser.get_curr_token().token_type != TokenType::RightParen {
            if parser.peek_token().token_type != TokenType::RightParen {
                panic!("expected ')' after grouped expression");
            }
            parser.advance_token();
        }

        next_expression
    }
}
