pub enum Object {
    Integer(i64),
    // String(String),
    Boolean(bool),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            // Self::String(s) => s.clone(),
            Self::Boolean(b) => b.to_string(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            Self::Integer(_) => "integer".to_string(),
            // Self::String(_) => "string".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
        }
    }
}
