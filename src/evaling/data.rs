
use std::collections::HashMap;

use purple::data::*;

use crate::runtime::*;

pub struct Context {
    pub address_map : HashMap<String, HeapAddress>,
    pub functions : HashMap<Func, Vec<Instr<RuntimeData, HashMap<HeapAddress, RuntimeData>>>>,
    pub heap : HashMap<HeapAddress, RuntimeData>,
}

impl Context {
    pub fn new() -> Self { 
        Context { address_map: HashMap::new(), functions: HashMap::new(), heap: HashMap::new() }
    }
}
