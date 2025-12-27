use crate::expander::ast::{Node, expression::Expression};

pub struct FunctionCallExpression {
    function_name: String,
    args: Vec<Box<Expression>>,
}

impl FunctionCallExpression {
    pub fn literal(&self) -> String {
        format!(
            "{}({})",
            self.function_name,
            self.args
                .iter()
                .map(|x| { x.token_literal() })
                .collect::<Vec<String>>()
                .join(",")
        )
    }

    pub fn new(function_name: String) -> Self {
        Self {
            function_name,
            args: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: Box<Expression>) {
        self.args.push(arg);
    }
}
