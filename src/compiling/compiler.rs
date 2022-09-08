use std::collections::HashMap;

use purple::data::*;
use crate::runtime::*;
use crate::ast::*;

use super::error::CompileError;


type I = Instr<RuntimeData, HashMap<HeapAddress, RuntimeData>>;
type Fs = HashMap<Func, Vec<I>>; 
type M = HashMap<String, HeapAddress>;

struct C {
    sym_count : usize,
    label_count : usize,
    func_count : usize,
}

impl C {
    fn new(func_count : usize) -> Self {
        C { sym_count: 0, label_count: 0, func_count }
    }
    fn fresh(&self) -> Self {
        C { sym_count: 0, label_count: 0, func_count: self.func_count }
    }
    fn symbol(&mut self) -> Symbol {
        self.sym_count += 1;
        Symbol(self.sym_count)
    }
    fn label(&mut self) -> Label {
        self.label_count += 1;
        Label(self.label_count)
    }
    fn func(&mut self) -> Func {
        self.func_count += 1;
        Func(self.func_count)
    }
}

pub fn compile(input : &Expr, address_map : &M, functions : &mut Fs) -> Result<Vec<I>, CompileError> {
    let mut func = functions.keys().map(|k| k.0).max().unwrap_or(0);
    let mut c = C::new(func);
    match input {
        Expr::Literal(lit) => { },
        Expr::Call(func_expr, params) => { },
    }
    Err(CompileError::Todo)
}

fn compile_literal(c : &mut C, input : &Lit, address_map : &M, functions : &mut Fs) -> Result<(Symbol, Vec<I>), CompileError> {
    match input {
        Lit::Number(x) => { 
            let s = c.symbol();
            return Ok((s, vec![
                Instr::LoadValue(s, RuntimeData::Number(*x))
            ]));
        },
        Lit::String(x) => { },
        Lit::Symbol(x) => { },
        Lit::Variable(x) => { },
        Lit::List(x) => { },
        Lit::Tuple(x) => { },
        Lit::Lambda(x) => { },
    }

    Err(CompileError::Todo)
}
