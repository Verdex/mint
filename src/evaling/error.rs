

#[derive(Debug)]
pub enum RuntimeError {

}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MatchError::Error(x) => write!(f, "encountered error at:  {}", x),
            MatchError::ErrorEndOfFile => write!(f, "encountered end of file error"),
            MatchError::Fatal(x) => write!(f, "encountered fatal error at:  {}", x),
            MatchError::FatalEndOfFile => write!(f, "encountered fatal end of file" ),
        }
    }
}

impl std::error::Error for RuntimeError {}