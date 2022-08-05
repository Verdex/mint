
use crate::ast::{ Top
                , Data
                , Expr
                };

use super::data::Context;
use super::error::RuntimeError;
use super::pattern_matcher;

pub fn eval( input : Top, context : &mut Context ) -> Result<Option<Data>, RuntimeError> {

    for l in input.lets {
        if let Some(new_context) = pattern_matcher::pattern_match(&l.pattern, &eval_expr(l.expr, context)?, context)? {
            context.merge(new_context)?;
        }
        else {
            return Err(RuntimeError::PatternMatchFailed);
        }
    } 

    if let Some(expr) = input.expr {
        Ok(Some(eval_expr(expr, context)?))
    }
    else {
        Ok(None)
    }
}

fn eval_expr( expr : Expr, context : &Context ) -> Result<Data, RuntimeError> {
    match expr {
        Expr::Data(data) => Ok(data),
        Expr::Call(func, params) => panic!("TODO"),
        _ => panic!("TODO"),
    }
}