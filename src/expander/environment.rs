use crate::expander::object::Object;
use std::collections::hash_map::HashMap;

pub struct Environment {
    variables: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Object {
        match self.variables.get(name) {
            Some(Object::Integer(value)) => Object::Integer(*value),
            Some(Object::Boolean(value)) => Object::Boolean(*value),
            Some(Object::None) => Object::None,
            Some(Object::Function(_)) => {
                panic!("function values are not cloneable yet")
            }
            None => Object::None,
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.variables.insert(name, value);
    }
}
