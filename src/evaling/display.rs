
use crate::ast::Data;
use super::context::Context;

pub fn print_data(data : &Data, context : &Context) -> String {
    match data {
        Data::Number(n) => n.to_string(),
        _ => "err".into(),
    }
}