
use std::collections::HashMap;

use purple::data::*;

use crate::ast::*;
use crate::runtime::*;

use super::error::*;

pub fn pattern_match(data : Symbol, pattern : Pat, result : Symbol, var_to_sym : HashMap<String, Symbol>) -> Instr<RuntimeData, Heap> {
    use crate::evaling::pattern_matcher::*; 

    Instr::<RuntimeData, Heap>::LoadFromSysCall(result, Box::new(
        move |locals, heap| {
            let data = {
                match locals.get(&data)? {
                    Data::Func(f) => RuntimeData::Function(f),
                    Data::Value(RuntimeData::Address(address)) =>
                        heap.get(address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?.clone(),
                    Data::Value(v) => v,
                }
            };

            match pattern_match(&pattern, &data) {
                MatchResult::Fatal(err) => { return Err(Box::new(err))},
                MatchResult::NoMatch => Ok(Data::Value(RuntimeData::Symbol("false".into()))),
                MatchResult::Env(bounds) => {
                    for BoundData { name, data } in bounds {
                        let address = heap.insert_new(data);
                        let sym = var_to_sym.get(&name).expect("var_to_sym missing variable");
                        locals.set(sym, Data::Value(RuntimeData::Address(address)))?;
                    }
                    Ok(Data::Value(RuntimeData::Symbol("true".into())))
                },
            }
        }))
}

pub fn load_from_heap(address : HeapAddress, sym : Symbol) -> Instr<RuntimeData, Heap> {
    Instr::<RuntimeData, Heap>::LoadFromSysCall(sym, Box::new(
        move |locals, heap| {
            let ret = heap.get(address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?;
                match ret {
                    RuntimeData::Function(f) => Ok(Data::Func(f.clone())),
                    data => Ok(Data::Value(data.clone()))
                }
            }))
}

pub fn insert_into_heap(symbol_to_insert : Symbol, return_symbol_for_address : Symbol) -> Instr<RuntimeData, Heap> {
    Instr::<RuntimeData, Heap>::LoadFromSysCall(return_symbol_for_address, Box::new(
        move |locals, heap| {
            match locals.get(&symbol_to_insert)? {
                Data::Value(value) => {
                    let address = heap.insert_new(value);
                    Ok(Data::Value(RuntimeData::Address(address)))
                },
                Data::Func(f) => {
                    let address = heap.insert_new(RuntimeData::Function(f));
                    Ok(Data::Value(RuntimeData::Address(address)))
                },
            }
        }
    ))
}

pub fn push_into_tuple_in_heap(item : Symbol, tuple_address : Symbol) -> Instr::<RuntimeData, Heap> { 

    fn error(expected : &str, observed : &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(DynamicError::TypeMismatch { expected: expected.into(), observed: observed.into()}))
    }

    Instr::<RuntimeData, Heap>::SysCall(Box::new(
        move |locals, heap| {
            let push_into_tuple = match locals.get(&item)? {
                Data::Value(x) => x,
                _ => { return error("Data::Value", "Data::Func"); }
            };

            let tuple_address = match locals.get(&tuple_address)? {
                Data::Value(RuntimeData::Address(x)) => x,
                Data::Value(_) => { return error("Data::Value(RuntimeData::Address", "Data::Value(?)"); },
                Data::Func(_) => { return error("Data::Value(RuntimeData::Address)", "Data::Func"); },
            };

            let tuple = heap.get_mut(tuple_address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?;

            if let RuntimeData::Tuple(t) = tuple {
                t.push(push_into_tuple);
                Ok(())
            }
            else {
                error( "RuntimeData::Tuple", "?" )
            }
        }
    ))
}

pub fn push_into_list_in_heap(item : Symbol, list_address : Symbol) -> Instr::<RuntimeData, Heap> { 

    fn error(expected : &str, observed : &str) -> Result<(), Box<dyn std::error::Error>> {
        Err(Box::new(DynamicError::TypeMismatch { expected: expected.into(), observed: observed.into()}))
    }

    Instr::<RuntimeData, Heap>::SysCall(Box::new(
        move |locals, heap| {
            let push_into_list = match locals.get(&item)? {
                Data::Value(x) => x,
                _ => { return error("Data::Value", "Data::Func"); }
            };

            let list_address = match locals.get(&list_address)? {
                Data::Value(RuntimeData::Address(x)) => x,
                Data::Value(_) => { return error("Data::Value(RuntimeData::Address", "Data::Value(?)"); },
                Data::Func(_) => { return error("Data::Value(RuntimeData::Address)", "Data::Func"); },
            };

            let list = heap.get_mut(list_address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?;

            if let RuntimeData::List(l) = list {
                l.push(push_into_list);
                Ok(())
            }
            else {
                error( "RuntimeData::List", "?" )
            }
        }
    ))
}