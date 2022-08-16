

use crate::ast::{ Top
                , Data
                , Expr
                , Lambda 
                , Let
                };

use super::data::Environment;

use super::error::CompileError;

pub fn compile(t : Top, environment : &mut Environment) -> Result<(), CompileError> {

    Ok(())
}

//fn compile_let(l : Let, environment : &mut Environment) -> Result<
