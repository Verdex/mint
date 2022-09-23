
use purple::data::*;

#[derive(Debug)]
pub enum StaticError {
    Todo
}

impl std::fmt::Display for StaticError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StaticError::Todo => write!(f, "TODO"),
        }
    }
}

impl std::error::Error for StaticError {}


#[derive(Debug)]
pub enum DynamicError {
    CannotFindLocal(Symbol),
}

impl std::fmt::Display for DynamicError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DynamicError::CannotFindLocal(sym) => write!(f, "cannot find local {}", sym.0),
        }
    }
}

impl std::error::Error for DynamicError {}