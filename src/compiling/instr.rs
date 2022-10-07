
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
