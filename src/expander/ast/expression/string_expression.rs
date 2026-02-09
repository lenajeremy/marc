pub struct StringExpression {
    value: String,
}

impl StringExpression {
    pub fn literal(&self) -> String {
        format!("String({})", self.value)
    }

    pub fn new(value: String) -> Self {
        Self { value }
    }
}
