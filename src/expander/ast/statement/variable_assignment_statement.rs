use crate::expander::ast::Node;
use crate::expander::ast::expression::Expression;
use crate::expander::environment::Environment;

pub struct VariableAssignmentStatement {
    identifier: String,
    value: Box<Expression>,
}

impl VariableAssignmentStatement {
    pub fn new(identifier: String, value: Box<Expression>) -> Self {
        Self { identifier, value }
    }

    pub fn literal(&self) -> String {
        format!(
            "VariableAssignmentStatement(\"{} = {}\")",
            self.identifier,
            self.value.token_literal()
        )
    }
}

impl Node for VariableAssignmentStatement {
    fn token_literal(&self) -> String {
        self.literal()
    }

    fn translate(&self, env: &mut Environment) -> String {
        let expression_result = self.value.evaluate(env);
        env.set(self.identifier.clone(), expression_result);

        "".to_string() // no visible text output should be returned after assigning a variable
    }

    fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}
