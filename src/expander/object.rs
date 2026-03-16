use crate::expander::ast::statement::FunctionDefinitionStatement;
use std::rc::Rc;

#[derive(Clone)]
pub enum Object {
    Integer(i64),
    Function(Rc<FunctionDefinitionStatement>),
    // String(String),
    Boolean(bool),
    None,
}

pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);
pub const NONE: Object = Object::None;

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            Self::Function(_) => "<function>".to_string(),
            // Self::String(s) => s.clone(),
            Self::Boolean(b) => b.to_string(),
            Self::None => "none".to_string(),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            Self::Integer(_) => "integer".to_string(),
            Self::Function(_) => "function".to_string(),
            // Self::String(_) => "string".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::None => "none".to_string(),
        }
    }
}
