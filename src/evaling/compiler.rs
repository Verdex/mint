

use crate::ast::{ Top
                , Data
                , Expr
                , Lambda 
                , Let
                };

use super::data::Environment;


pub fn compile(t : Top, environment : &mut Environment) -> Result<(), CompileError> {

    Ok(())
}

//fn compile_let(l : Let, environment : &mut Environment) -> Result<
