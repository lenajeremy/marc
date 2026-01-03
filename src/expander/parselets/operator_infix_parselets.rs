use crate::expander::{
    ast::{
        expression::{Expression, InfixExpression},
        operators::Op,
    },
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precendence,
    token::{Token, TokenType},
};

pub struct OperatorInfixParselet;

impl InfixParselet for OperatorInfixParselet {
    fn get_precedence(&self, token: Token) -> u8 {
        let precendence = match token.token_type {
            TokenType::Plus | TokenType::Minus => Precendence::SUM,
            TokenType::Asterisk | TokenType::ForwardSlash => Precendence::PRODUCT,
            _ => panic!(
                "Expected math operator token ('+', '-', '/', '*'), got {}",
                token.literal
            ),
        };

        precendence as u8
    }

    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression> {
        let token = parser.get_curr_token();

        let operator = Op::from_token(&token)
            .expect(format!("expected a valid operator token, got {}", token.literal).as_str());

        parser.advance_token();

        let next_expression = parser.parse_expression(self.get_precedence(parser.get_curr_token()));

        Box::new(Expression::OperatorInfix(InfixExpression::new(
            left,
            next_expression,
            operator,
        )))
    }
}
