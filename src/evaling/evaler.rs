
use crate::ast::{ Top
                , Lit 
                , Expr
                };

use super::data::{ Context
                 , RuntimeData
                 , FunctionAddress
                 , RuntimeDataAddress
                 , InstructionIndex
                 };
use super::error::RuntimeError;
use super::pattern_matcher;

// TODO:  Need a stack for contexts and the index of the instruction vector of the calling function

pub fn eval_2(main : FunctionAddress, context : RuntimeDataAddress) -> Result<Option<RuntimeData>, RuntimeError> {
    Ok(None)
}

pub fn eval( input : Top, context : &mut Context ) -> Result<Option<Lit>, RuntimeError> {

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

fn eval_expr( expr : Expr, context : &Context ) -> Result<Lit, RuntimeError> {
    match expr {
        Expr::Literal(data) => Ok(data),
        Expr::Call(func, params) => panic!("TODO"),
        _ => panic!("TODO"),
    }
}