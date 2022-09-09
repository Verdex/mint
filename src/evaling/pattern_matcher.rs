
use std::collections::HashMap;

use crate::ast::{Pat, Lit};
use crate::runtime::*;

use super::error::*;

pub enum MatchResult {
    Env(Vec<BoundData>),
    NoMatch,
    Fatal(RuntimeError),
}

pub struct BoundData {
    name : String,
    data : RuntimeData,
}

    /*Address(HeapAddress),
    Function(Func),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<RuntimeData>),
    Tuple(Vec<RuntimeData>),*/


    /*Wild,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Pat>, Option<Box<Pat>>),
    Tuple(Vec<Pat>),
    Variable(String),
    At(String, Box<Pat>),*/

pub fn pattern_match( pattern : &Pat, data : &RuntimeData ) -> MatchResult {
    use MatchResult::*;
    use std::iter::zip;
    match (pattern, data) {
        (Pat::Wild, _) => Env(vec![]),
        (Pat::Number(a), RuntimeData::Number(b)) if a == b => Env(vec![]),
        (Pat::String(a), RuntimeData::String(b)) if a == b => Env(vec![]),
        (Pat::Symbol(a), RuntimeData::Symbol(b)) if a == b => Env(vec![]),
        (Pat::Tuple(a), RuntimeData::Tuple(b)) if a.len() != b.len() => NoMatch, 
        (Pat::Tuple(a), RuntimeData::Tuple(b)) => {
            let mut all = HashMap::new();
            for (pat, data) in zip(a, b) {
                match pattern_match(pat, data) {
                    NoMatch => { return NoMatch; },
                    Fatal(e) => { return Fatal(e); },
                    Env(env) => {
                        for e in env {
                            if all.contains_key(&e.name) {
                                return Fatal(RuntimeError::CannotSetBoundVariable(e.name));
                            }
                            all.insert(e.name, e.data);
                        }
                    },
                }
            }
            Env(all.into_iter().map(|kvp| BoundData { name: kvp.0, data: kvp.1 }).collect::<Vec<_>>())
        },
        _ => NoMatch,
    }
}

/*pub fn pattern_match( pattern : &Pat, data : &RuntimeData, context : &Context ) -> Result<Option<Context>, RuntimeError> {
    match (pattern, data) {
        (Pat::Wild, _) => Ok(Some(Context::new())),

        (Pat::Number(p_num), Lit::Number(d_num)) if p_num == d_num => Ok(Some(Context::new())),

        (Pat::String(p_str), Lit::String(d_str)) if p_str == d_str => Ok(Some(Context::new())),

        (Pat::Symbol(p_sym), Lit::Symbol(d_sym)) if p_sym == d_sym => Ok(Some(Context::new())),

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

        (Pat::Tuple(pats), Lit::Tuple(datas)) if pats.len() != datas.len() => Ok(None),
        (Pat::Tuple(pats), Lit::Tuple(datas)) => {
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

        (pat, Lit::Variable(var)) => pattern_match(pat, &context.lookup(var)?, context),

        (Pat::List(pats, _), Lit::List(datas)) if pats.len() > datas.len() => Ok(None),
        (Pat::List(pats, Some(rest)), Lit::List(datas)) => { 
            let mut list_context = Context::new();
            let mut rest_data = vec![];
            for i in 0..datas.len() {
                if i < pats.len() {
                    if let Some(new_context) = pattern_match(&pats[i], &datas[i], context)? {
                        list_context.merge(new_context)?;
                    }
                    else {
                        return Ok(None);
                    }
                }
                else {
                    rest_data.push(datas[i].clone()); // NOTE:  Probably not great for memory usage
                }
            }
            if let Some(new_context) = pattern_match(rest, &Lit::List(rest_data), context)? {
                list_context.merge(new_context)?;
            }
            else {
                return Ok(None);
            }
            Ok(Some(list_context))
        },
        (Pat::List(pats, None), Lit::List(datas)) => {
            let mut list_context = Context::new();
            for (p, d) in std::iter::zip(pats, datas) {
                if let Some(new_context) = pattern_match(p, d, context)? {
                    list_context.merge(new_context)?;
                }
                else {
                    return Ok(None);
                }
            }
            Ok(Some(list_context))
        },

        _ => Ok(None) 

    }
}*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo() {

    }
}