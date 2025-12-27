pub struct VariableAccessExpression {
    variable_name: String,
}

impl VariableAccessExpression {
    pub fn literal(&self) -> String {
        self.variable_name.clone()
    }

    pub fn new(variable_name: String) -> Self {
        VariableAccessExpression {
            variable_name: variable_name,
        }
    }
}
