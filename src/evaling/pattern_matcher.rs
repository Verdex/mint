
use crate::ast::{Pat, Data};
use super::context::Context;
use super::error::RuntimeError;

/*

pub enum Pat {
    Wild,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Pat>, Option<Box<Pat>>),
    Tuple(Vec<Pat>),
    Variable(String),
    At(String, Box<Pat>),
}


pub enum Data {
    Number(f64),
    String(String),
    Symbol(String),
    Variable(String),
    List(Vec<Data>),
    Tuple(Vec<Data>),
    Lambda(Vec<Pat>, Box<Top>),
}

*/

// TODO (remove) NOTE Err is runtime issue; Ok(None) is fail to pattern match
pub fn pattern_match( pattern : &Pat, data : &Data, context : &Context ) -> Result<Option<Context>, RuntimeError> {
    match (pattern, data) {
        (Pat::Wild, _) => Ok(Some(Context::new())),

        (Pat::At(var, pat), data) => {
            if let Some(new_context) = pattern_match(pat, data, context)? {
                let mut var_context = Context::new();
                var_context.set(var, data.clone());
                var_context.merge(new_context)?;
                Ok(Some(var_context))
            }
            else {
                Ok(None)
            }
        },

        (Pat::Number(p_num), Data::Number(d_num)) if p_num == d_num => {
            Ok(None) // TODO
        },
        (Pat::Number(p_num), data) => {

            Ok(None) // TODO
        },

        (Pat::String(p_str), Data::String(d_str)) if p_str == d_str => {

            Ok(None) // TODO
        },
        (Pat::String(p_str), data) => {

            Ok(None) // TODO
        },

        (Pat::Symbol(p_sym), Data::Symbol(d_sym)) if p_sym == d_sym => {

            Ok(None) // TODO
        },
        (Pat::Symbol(p_sym), data) => {

            Ok(None) // TODO
        },

        _ => Ok(None) // TODO:  not sure what _ should be in this context


        // TODO Tuple
        // TODO Variable
        // TODO List
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo() {

    }
}