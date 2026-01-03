use crate::expander::{
    ast::expression::{Expression, VariableAccessExpression},
    parselets::PrefixParselet,
    parser::Parser,
    precedence::Precendence,
    token::{Token, TokenType},
};

pub struct VariableAccessParselet;

impl PrefixParselet for VariableAccessParselet {
    fn get_precedence(&self) -> u8 {
        Precendence::VARIABLE as u8
    }

    fn parse_expression(&self, _: &mut Parser, token: Token) -> Box<Expression> {
        println!("parsing a variable with token {token:?}");
        if token.token_type == TokenType::Identifier {
            Box::new(Expression::VariableAccess(VariableAccessExpression::new(
                token.literal,
            )))
        } else {
            panic!("expected an identifier token, got {}", token.literal);
        }
    }
}
