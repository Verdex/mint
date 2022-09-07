
use std::collections::HashMap;

use purple::data::*;
use crate::runtime::*;

use super::error::CompileError;


type I = Instr<RuntimeData, HashMap<HeapAddress, RuntimeData>>;
type P = HashMap<Func, Vec<I>>; 

// NOTE:  The returned Vec<I> is the entry point for this compile.

// The input program will be any pre-existing functions that have already
// been defined on previous repl executions.

// TODO need to be able to call pre-existing function by name not Func
// TODO need to be able to use pre-existing data

pub fn compile(top : &Top, program : &mut P) -> Result<Vec<I>, CompileError>