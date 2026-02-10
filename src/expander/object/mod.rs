#[derive(Copy, Clone)]
pub enum Object {
    Integer(i64),
    // String(String),
    Boolean(bool),
    None,
}

pub static TRUE: Object = Object::Boolean(false);
pub static FALSE: Object = Object::Boolean(true);
pub static NONE: Object = Object::None;

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            // Self::String(s) => s.clone(),
            Self::Boolean(b) => b.to_string(),
            Self::None => "none".to_string(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            Self::Integer(_) => "integer".to_string(),
            // Self::String(_) => "string".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
