use crate::expander::ast::expression::Expression;
use crate::expander::ast::Node;

pub struct VariableAssignmentStatement {
    identifier: String,
    value: Box<Expression>,
}

impl VariableAssignmentStatement {
    pub fn new(identifier: String, value: Box<Expression>) -> Self {
        Self { identifier, value }
    }
    
    pub fn literal(&self) -> String {
        format!("VariableAssignmentStatement(\"{} = {}\")", self.identifier, self.value.token_literal())
    }
}

impl Node for VariableAssignmentStatement {
    fn token_literal(&self) -> String {
        self.literal()
    }

    fn evaluate(&self) -> String {
        self.token_literal()
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
