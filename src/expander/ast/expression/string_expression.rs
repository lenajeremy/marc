pub struct StringExpression {
    value: String,
}

impl StringExpression {
    pub fn literal(&self) -> String {
        self.value.to_owned()
    }
    
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
