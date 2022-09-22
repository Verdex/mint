
use std::collections::HashMap;

use purple::data::*;

use crate::runtime::*;


pub struct Context {
    pub address_map : HashMap<String, HeapAddress>,
    pub functions : HashMap<Func, Vec<Instr<RuntimeData, Heap>>>,
    pub heap : Heap 
}

impl Context {
    pub fn new() -> Self { 
        Context { address_map: HashMap::new(), functions: HashMap::new(), heap: Heap::new() }
    }
}
