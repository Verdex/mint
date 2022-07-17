
use std::collections::HashMap;
use crate::ast::Data;
use super::error::RuntimeError;

pub struct Context {
    bound_variables : HashMap<String, Data>,
}

impl Context {
    pub fn new() -> Self {
        Context { bound_variables : HashMap::new() }
    }

    pub fn lookup(&self, name : &str) -> Result<Data, RuntimeError> {
        match self.bound_variables.get(name) {
            Some(data) => Ok(data.clone()),
            None => Err(RuntimeError::VariableNotFound(name.into())),
        }
    }

}