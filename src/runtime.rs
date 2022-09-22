
use std::collections::HashMap;

use purple::data::Func;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HeapAddress(u64);

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeData {
    Address(HeapAddress),
    Function(Func),
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<RuntimeData>),
    Tuple(Vec<RuntimeData>),
}

pub struct Heap {
    heap : HashMap<HeapAddress, RuntimeData>,
    new_address : u64,
}

impl Heap {
    pub fn new() -> Self {
        Heap { heap : HashMap::new(), new_address : 0 }
    }

    pub fn get(&mut self, address : HeapAddress) -> Option<&mut RuntimeData> {
        self.heap.get_mut(&address)
    }

    pub fn insert_new(&mut self, data : RuntimeData) -> HeapAddress {
        let ret = HeapAddress(self.new_address);
        self.heap.insert(ret, data);
        self.new_address += 1;
        ret
    }

    pub fn insert(&mut self, address : HeapAddress, data : RuntimeData) {
        self.heap.insert(address, data);
    }
}