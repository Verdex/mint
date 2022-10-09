
use purple::data::*;

use crate::runtime::*;

use super::error::*;

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