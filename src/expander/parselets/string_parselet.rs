use crate::expander::{
    ast::expression::{Expression, StringExpression},
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precedence,
    token::{Token, TokenType},
};

pub struct StringParselet;

impl PrefixParselet for StringParselet {
    fn get_precedence(&self) -> u8 {
        Precedence::VARIABLE as u8
    }

    fn parse_expression(&self, parser: &mut Parser, token: Token) -> Box<Expression> {
        if token.token_type != TokenType::DoubleQuote && token.token_type != TokenType::SingleQuote
        {
            panic!("expected a string quote token, got {}", token.literal);
        }

        let string_value = parser.parse_quoted_string(token.token_type);
        Box::new(Expression::String(StringExpression::new(string_value)))
    }
}
