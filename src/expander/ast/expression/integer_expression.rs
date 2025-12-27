pub struct IntegerExpression {
    value: i64,
}

impl IntegerExpression {
    pub fn literal(&self) -> String {
        format!("Integer({})", self.value)
    }
}
