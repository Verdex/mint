

use purple::data::*;


use crate::ast::*;
use crate::compiling::compiler;

use super::data::Context;
use super::error::RuntimeError;
use super::pattern_matcher;


pub fn eval( input : Top, context : &mut Context ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    for l in input.lets {
        let program = compiler::compile(&l.expr, &context.address_map, &mut context.functions)?;
        // Note:  We can leave functions alone after we're done because the next eval will flush Func(0)
        context.functions.insert(Func(0), program); 
        let result = purple::run(&context.functions, &mut context.heap)?;
        // TODO: pattern match result, stuff things into address_map (when do things need to be pulled out?)
    }
    Ok(None)
}
