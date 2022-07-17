

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotFound(String),
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::VariableNotFound(s) => write!(f, "could not find variable:  {}", s),
        }
    }
}

impl std::error::Error for RuntimeError {}