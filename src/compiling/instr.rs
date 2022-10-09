
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