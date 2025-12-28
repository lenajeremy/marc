use crate::expander::ast::{Node, expression::Expression};

pub struct FunctionCallExpression {
    function_identifier: Box<Expression>,
    args: Vec<Box<Expression>>,
}

impl FunctionCallExpression {
    pub fn literal(&self) -> String {
        format!(
            "FunctionCall({}({}))",
            self.function_identifier.token_literal(),
            self.args
                .iter()
                .map(|x| { x.token_literal() })
                .collect::<Vec<String>>()
                .join(", ")
        )
    }

    pub fn new(identifier: Box<Expression>) -> Self {
        Self {
            function_identifier: identifier,
            args: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: Box<Expression>) {
        self.args.push(arg);
    }
}
