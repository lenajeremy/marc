use crate::expander::{
    ast::expression::{Expression, FunctionCallExpression},
    parselets::InfixParselet,
    parser::Parser,
    precedence::Precendence,
    token::{Token, TokenType},
};

pub struct FunctionCallParselet;

impl InfixParselet for FunctionCallParselet {
    fn get_precedence(&self, _token: Token) -> u8 {
        Precendence::CALL as u8
    }

    fn parse_expression(
        &self,
        parser: &mut Parser,
        identifier: Box<Expression>,
    ) -> Box<Expression> {
        parser.advance_token(); // move the cursor past the `(` char to the next token.

        let args = self.parse_function_args(parser);
        let mut function_call_expression = FunctionCallExpression::new(identifier);

        for arg in args {
            function_call_expression.add_arg(arg);
        }

        Box::new(Expression::FunctionCall(function_call_expression))
    }
}

impl FunctionCallParselet {
    fn parse_function_args(&self, parser: &mut Parser) -> Vec<Box<Expression>> {
        let mut args = Vec::new();

        loop {
            match parser.get_curr_token().token_type {
                TokenType::RightParen | TokenType::EOF => break,
                TokenType::Comma => {
                    parser.advance_token();
                    continue;
                }
                _ => {
                    let expression =
                        parser.parse_expression(self.get_precedence(parser.get_curr_token()));
                    args.push(expression);
                }
            };
        }

        args
    }
}
