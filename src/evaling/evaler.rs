
use crate::ast::{ Top
                , Data
                , Expr
                };

use super::context::Context;

pub fn eval( input : Top, context : &mut Context ) -> Result<Data, String> {
    // TODO resolve lets
    match input.expr {
        Expr::Data(data) => Ok(data),
        _ => panic!("!"),
    }
}