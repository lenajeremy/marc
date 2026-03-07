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
        self.variables.get(name).cloned().unwrap_or(Object::None)
    }

    fn set(&mut self, name: String, value: Object) -> Object {
        self.variables.insert(name, value);
        value
    }
}
