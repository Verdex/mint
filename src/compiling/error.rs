
#[derive(Debug)]
pub enum CompileError {
    Todo,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::Todo => write!(f, "TODO"),
        }
    }
}

impl std::error::Error for CompileError {}
