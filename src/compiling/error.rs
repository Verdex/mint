
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
    TypeMismatch { expected : String, observed : String },
    CannotFindHeapAddress,
}

impl std::fmt::Display for DynamicError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DynamicError::TypeMismatch { expected, observed } => write!(f, "type mismatch expected {}, but found {}", expected, observed),
            DynamicError::CannotFindHeapAddress => write!(f, "cannot find heap address"),
        }
    }
}

impl std::error::Error for DynamicError {}