
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

fn data_to_pattern(data : &Data) -> Result<Pat, RuntimeError> {
    match data {
        Data::Number(num) => Ok(Pat::Number(*num)),
        Data::String(s) => Ok(Pat::String(s.into())),
        Data::Symbol(s) => Ok(Pat::Symbol(s.into())),
        Data::Variable(v) => Ok(Pat::Variable(v.into())), 
        Data::List(datas) => Ok(Pat::List( datas.iter().map(data_to_pattern).collect::<Result<_, RuntimeError>>()?, None)),
        Data::Tuple(datas) => Ok(Pat::Tuple(datas.iter().map(data_to_pattern).collect::<Result<_, RuntimeError>>()?)),
        Data::Lambda(_, _) => Err(RuntimeError::CannotPatternMatchAgainstLambda),
    }
}

// TODO (remove) NOTE Err is runtime issue; Ok(None) is fail to pattern match
pub fn pattern_match( pattern : &Pat, data : &Data, context : &Context ) -> Result<Option<Context>, RuntimeError> {
    match (pattern, data) {
        (Pat::Wild, _) => Ok(Some(Context::new())),

        (Pat::Number(p_num), Data::Number(d_num)) if p_num == d_num => Ok(Some(Context::new())),

        (Pat::String(p_str), Data::String(d_str)) if p_str == d_str => Ok(Some(Context::new())),

        (Pat::Symbol(p_sym), Data::Symbol(d_sym)) if p_sym == d_sym => Ok(Some(Context::new())),

        (Pat::At(var, pat), data) => {
            if let Some(new_context) = pattern_match(pat, data, context)? {
                let mut var_context = Context::new();
                var_context.set(var, data.clone())?;
                var_context.merge(new_context)?;
                Ok(Some(var_context))
            }
            else {
                Ok(None)
            }
        },

        (Pat::Tuple(pats), Data::Tuple(datas)) => {
            if pats.len() != datas.len() {
                Ok(None)
            }
            else {
                let mut tuple_context = Context::new();
                for (p, d) in std::iter::zip(pats, datas) {
                    if let Some(new_context) = pattern_match(p, d, context)? {
                        tuple_context.merge(new_context)?;
                    }
                    else {
                        return Ok(None);
                    }
                }
                Ok(Some(tuple_context))
            }
        },

        (Pat::Variable(var), data) => {
            match context.lookup(var) {
                // bound pattern variable means match against whatever is in there
                Ok(p) => pattern_match(&data_to_pattern(&p)?, data, context), 
                // unbound pattern variable means capture whatever is there
                Err(RuntimeError::VariableNotFound(_)) => {
                    let mut new_context = Context::new();
                    new_context.set(var, data.clone())?;
                    Ok(Some(new_context))
                },
                Err(e) => Err(e),
            }
        },

        (pat, Data::Variable(var)) => pattern_match(pat, &context.lookup(var)?, context),

        _ => Ok(None) 


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