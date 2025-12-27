pub struct IntegerExpression {
    value: i64,
}

impl IntegerExpression {
    pub fn literal(&self) -> String {
        format!("Integer({})", self.value)
    }

    pub fn new(value: i64) -> Self {
        Self { value }
    }
}
