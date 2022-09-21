
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
    pub name : String,
    pub data : RuntimeData,
}

pub fn pattern_match( pattern : &Pat, data : &RuntimeData ) -> MatchResult {
    use MatchResult::*;
    use std::iter::{zip, repeat};
    match (pattern, data) {
        (Pat::Wild, _) => Env(vec![]),
        (Pat::Variable(a), b) => Env(vec![BoundData{ name: a.clone(), data: b.clone()}]),
        (Pat::Number(a), RuntimeData::Number(b)) if a == b => Env(vec![]),
        (Pat::String(a), RuntimeData::String(b)) if a == b => Env(vec![]),
        (Pat::Symbol(a), RuntimeData::Symbol(b)) if a == b => Env(vec![]),
        (Pat::At(name, pat), b) => {
            match pattern_match(pat, b) {
                NoMatch => NoMatch,
                Fatal(e) => Fatal(e),
                Env(mut env) => {
                    if env.iter().any(|x| x.name == *name) {
                        return Fatal(RuntimeError::CannotSetBoundVariable(name.clone()));
                    }
                    env.push(BoundData{ name: name.clone(), data: b.clone() });
                    Env(env)
                }
            }
        },
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
        // NOTE:  If there exists more patterns than items in the target list, then indicate NoMatch.
        (Pat::List(a, _), RuntimeData::List(b)) if a.len() > b.len() => NoMatch,
        // NOTE:  If there is no 'rest' pattern, then the lengths need to match.
        (Pat::List(a, None), RuntimeData::List(b)) if a.len() != b.len() => NoMatch,
        (Pat::List(a, None), RuntimeData::List(b)) => {
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
        (Pat::List(a, Some(rest)), RuntimeData::List(b)) => {
            let p = a.iter().map(|x| Some(x)).chain(repeat(None));
            let (matches, rest_data) : (Vec<_>, Vec<_>) = zip(p, b).partition(|(a, _)| a.is_some());

            let mut all = HashMap::new();
            for (pat, data) in matches {
                match pattern_match(pat.unwrap(), data) {
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

            let rest_data = rest_data.into_iter().map(|(_, b)| b.clone()).collect();

            match pattern_match(rest, &RuntimeData::List(rest_data)) {
                NoMatch => { return NoMatch; },
                Fatal(e) => { return Fatal(e); },
                Env(env) => {
                    for e in env {
                        if all.contains_key(&e.name) {
                            return Fatal(RuntimeError::CannotSetBoundVariable(e.name));
                        }
                        all.insert(e.name, e.data);
                    }
                }
            }
            Env(all.into_iter().map(|kvp| BoundData { name: kvp.0, data: kvp.1 }).collect::<Vec<_>>())
        },
        _ => NoMatch,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo() {

    }
}