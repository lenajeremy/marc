use crate::expander::{ast::expression::Expression, parser::Parser, token::Token};

// the Token arg is passed to the function can match the PrefixParseleetFn type signature
pub fn parse_grouped_expression(parser: &mut Parser, _: Token) -> Box<Expression> {
    parser.advance_token(); // move the cursor past the `(` char to the next token.

    let next_expression = parser.parse_expression();

    next_expression
}
