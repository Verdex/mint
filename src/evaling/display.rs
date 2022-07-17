
use crate::ast::Data;
use super::context::Context;
use super::error::RuntimeError;

pub fn print_data(data : &Data, context : &Context) -> Result<String, RuntimeError> {
    match data {
        Data::Number(n) => Ok(n.to_string()),
        Data::String(s) => Ok(format!("\"{}\"", s)),
        Data::Symbol(s) => Ok(s.into()),
        _ => Ok("err".into()),
    }
}