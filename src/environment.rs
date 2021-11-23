use std::collections::HashMap;

use crate::object::Object;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, obj: Object) -> Object {
        self.store.insert(name.to_string(), obj.clone());
        obj
    }
}
