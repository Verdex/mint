
use crate::ast::{ Top
                , Data
                , Expr
                };

use super::context::Context;

pub fn print_data(data : &Data, context : &Context) -> String {
    "err".into()
}

pub fn eval( input : Top, context : &mut Context ) -> Result<Data, String> {
    match input.expr {
        Expr::Data(data) => Ok(data),
        _ => panic!("!"),
    }
}