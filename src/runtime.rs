
use purple::data::Func;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct HeapAddress(pub u64);

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