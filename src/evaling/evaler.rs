
use purple::data::*;

use crate::ast::*;
use crate::runtime::*;
use crate::compiling::compiler;

use super::data::Context;
use super::error::RuntimeError;
use super::pattern_matcher::*;
use super::display::*;

pub fn eval( input : Top, context : &mut Context ) -> Result<Option<String>, Box<dyn std::error::Error>> {
    for l in input.lets {
        let program = compiler::compile(&l.expr, &context.address_map, &mut context.functions)?;
        // Note:  We can leave functions alone after we're done because the next eval will flush Func(0)
        context.functions.insert(Func(0), program); 
        let result = purple::run(&context.functions, &mut context.heap)?;

        if result.is_none() { 
            continue;
        }

        let data = match result.unwrap() { 
            Data::Value(v) => v,
            Data::Func(f) => RuntimeData::Function(f),
        };

        match pattern_match(&l.pattern, &data) {
            MatchResult::Fatal(e) => { return Err(Box::new(e)); },
            MatchResult::NoMatch => { return Err(Box::new(RuntimeError::PatternMatchFailed)); },
            MatchResult::Env(bound) => { 
                for b in bound {
                    if context.address_map.contains_key(&b.name) {
                        return Err(Box::new(RuntimeError::CannotSetBoundVariable(b.name)));
                    }

                    let address = context.heap.insert_new(b.data);
                    context.address_map.insert(b.name, address);
                }
            },
        }
    }

    if input.expr.is_none() {
        Ok(None)
    }
    else {
        let program = compiler::compile(&input.expr.unwrap(), &context.address_map, &mut context.functions)?;
        // Note:  We can leave functions alone after we're done because the next eval will flush Func(0)
        context.functions.insert(Func(0), program); 
        let result = purple::run(&context.functions, &mut context.heap)?;
        match result {
            Some(Data::Value(v)) => Ok(Some(print_data(&v))),
            Some(Data::Func(f)) => Ok(Some(print_data(&RuntimeData::Function(f)))),
            None => Ok(None),
        }
    }
}
