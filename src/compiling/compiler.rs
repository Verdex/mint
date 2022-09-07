use std::collections::HashMap;

use purple::data::*;
use crate::runtime::*;
use crate::ast::*;

use super::error::CompileError;


type I = Instr<RuntimeData, HashMap<HeapAddress, RuntimeData>>;
type P = HashMap<Func, Vec<I>>; 

// NOTE:  The returned Vec<I> is the entry point for this compile.

// The input program will be any pre-existing functions that have already
// been defined on previous repl executions.

// The address_map maps strings to a value in the heap

pub fn compile(top : &Top, program : &mut P, address_map : &mut Heap<String, HeapAddress> ) -> Result<Vec<I>, CompileError> {
    Err(CompileError::Todo)
}