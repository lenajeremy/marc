use crate::expander::{
    ast::expression::{Expression, FunctionCallExpression},
    parser::Parser,
    token::TokenType,
};

pub fn parse_function_call_expression(
    parser: &mut Parser,
    function_name: String,
) -> Box<Expression> {
    parser.advance_token(); // move the cursor past the `(` char to the next token.

    let args = parse_function_args(parser);
    let mut function_call_expression = FunctionCallExpression::new(function_name);

    for arg in args {
        function_call_expression.add_arg(arg);
    }

    Box::new(Expression::FunctionCall(function_call_expression))
}

fn parse_function_args(parser: &mut Parser) -> Vec<Box<Expression>> {
    let mut args = Vec::new();

    loop {
        match parser.get_curr_token().token_type {
            TokenType::RightParen | TokenType::EOF => break,
            TokenType::Comma => continue,
            _ => {
                let expression = parser.parse_expression();
                args.push(expression);
            }
        }
    }

    args
}
