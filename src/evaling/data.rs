
use std::collections::HashMap;
use crate::ast::Data;
use super::error::RuntimeError;

#[derive(Debug)]
pub struct FunctionAddress(usize);
#[derive(Debug)]
pub struct InstructionIndex(usize);
#[derive(Debug)]
pub struct RuntimeDataAddress(usize);

#[derive(Debug)]
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

    pub fn set(&mut self, name : &str, data : Data) -> Result<(), RuntimeError> {
        match self.bound_variables.get(name) {
            None => { self.bound_variables.insert(name.into(), data); Ok(()) },
            Some(_) => Err(RuntimeError::CannotSetBoundVariable(name.into())),
        }
    }

    pub fn merge(&mut self, context : Context) -> Result<(), RuntimeError> {
        for (var, data) in context.bound_variables.into_iter() {
            self.set(&var, data)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Instruction {

}

#[derive(Debug)]
pub enum RuntimeData {
    Function(FunctionAddress, RuntimeDataAddress), 
    Context(Context),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<RuntimeDataAddress>),
    Tuple(Vec<RuntimeDataAddress>),
}