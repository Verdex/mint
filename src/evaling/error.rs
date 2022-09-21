

#[derive(Debug)]
pub enum RuntimeError {
    VariableNotFound(String),
    CannotSetBoundVariable(String),
    CannotPatternMatchAgainstLambda, // TODO:  This probably gets removed?
    PatternMatchFailed,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RuntimeError::VariableNotFound(s) => write!(f, "could not find variable:  {}", s),
            RuntimeError::CannotSetBoundVariable(s) => write!(f, "cannot set already bound variable:  {}", s),
            RuntimeError::CannotPatternMatchAgainstLambda => write!(f, "cannot pattern match against a lambda"),
            RuntimeError::PatternMatchFailed => write!(f, "pattern match failed"),
        }
    }
}

impl std::error::Error for RuntimeError {}