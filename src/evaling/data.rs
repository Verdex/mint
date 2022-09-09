
use std::collections::HashMap;

use purple::data::*;

use crate::runtime::*;

pub struct Context {
    address_map : HashMap<String, HeapAddress>,
    functions : HashMap<Func, Vec<Instr<RuntimeData, HashMap<HeapAddress, RuntimeData>>>>,
}

impl Context {
    pub fn new() -> Self { 
        Context { address_map: HashMap::new(), functions: HashMap::new() }
    }
}