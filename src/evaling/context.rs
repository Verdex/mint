
use std::collections::HashMap;

use crate::ast::Data;

pub struct Context {
    bound_variables : HashMap<String, Data>,
}

impl Context {
    pub fn new() -> Self {
        Context { bound_variables : HashMap::new() }
    }

    pub fn lookup(&self, name : &str) -> Option<Data> {
        match self.bound_variables.get(name) {
            Some(data) => Some(data.clone()),
            None => None,
        }
    }

}