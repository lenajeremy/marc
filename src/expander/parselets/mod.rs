use crate::expander::{ast::expression::Expression, parser::Parser, token::Token};

pub mod array_parselets;
pub mod bracket_expression_parselets;
pub mod function_call_parselet;
pub mod integer_parselet;
pub mod object_parselets;
pub mod operator_infix_parselets;
pub mod operator_prefix_parselets;
pub mod string_parselet;
pub mod variable_access_parselet;

pub trait PrefixParselet {
    fn get_precedence(&self) -> u8;
    fn parse_expression(&self, parser: &mut Parser, token: Token) -> Box<Expression>;
}

pub trait InfixParselet {
    fn get_precedence(&self, token: Token) -> u8;
    fn parse_expression(&self, parser: &mut Parser, left: Box<Expression>) -> Box<Expression>;
}
