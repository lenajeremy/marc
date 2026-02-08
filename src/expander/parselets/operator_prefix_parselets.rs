use crate::expander::{
    ast::{
        expression::{Expression, PrefixExpression},
        operators::Op,
    },
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precedence,
    token::Token,
};

pub struct OperatorPrefixParselet;

impl PrefixParselet for OperatorPrefixParselet {
    fn get_precedence(&self) -> u8 {
        Precedence::PREFIX as u8
    }

    fn parse_expression(&self, parser: &mut Parser, token: Token) -> Box<Expression> {
        let operator = Op::from_token(&token);
        if operator.is_none() {
            panic!("expected a valid operator token, got {}", token.literal);
        }

        let operator = operator.unwrap();
        parser.advance_token();
        let next_expression = parser.parse_expression(self.get_precedence());
        Box::new(Expression::Prefix(PrefixExpression::new(
            operator,
            next_expression,
        )))
    }
}
