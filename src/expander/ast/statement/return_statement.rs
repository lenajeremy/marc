use crate::expander::ast::{Node, expression::Expression};

pub struct ReturnStatement {
    value: Box<Expression>,
}

impl ReturnStatement {
    pub fn new(value: Box<Expression>) -> Self {
        Self { value }
    }

    pub fn literal(&self) -> String {
        format!("ReturnStatement(\"return {}\")", self.value.token_literal())
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.literal()
    }

    fn translate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
