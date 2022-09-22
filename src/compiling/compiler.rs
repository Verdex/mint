use std::collections::HashMap;

use purple::data::*;
use crate::runtime::*;
use crate::ast::*;

use super::error::CompileError;


type I = Instr<RuntimeData, Heap>;
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
        Expr::Literal(lit) => {
            let (sym, mut prog) = compile_literal(&mut c, &lit, address_map, functions)?;
            let mut x = vec![ Instr::Return(sym) ];
            prog.append(&mut x);
            Ok(prog)
        },
        Expr::Call(func_expr, params) => Err(CompileError::Todo),
    }
}

fn compile_literal(c : &mut C, input : &Lit, address_map : &M, functions : &mut Fs) -> Result<(Symbol, Vec<I>), CompileError> {
    match input {
        Lit::Number(x) => { 
            let s = c.symbol();
            Ok((s, vec![
                Instr::LoadValue(s, RuntimeData::Number(*x))
            ]))
        },
        Lit::String(x) => {
            let s = c.symbol();
            Ok((s, vec![
                Instr::LoadValue(s, RuntimeData::String(x.to_string()))
            ]))
        },
        Lit::Symbol(x) => {
            let s = c.symbol();
            Ok((s, vec![
                Instr::LoadValue(s, RuntimeData::Symbol(x.to_string()))
            ]))
        },
        Lit::Variable(x) => {
            Err(CompileError::Todo)
        },
        Lit::List(x) => {
            let y = x.iter().map(|d| compile_literal(c, d, address_map, functions)).collect::<Result<Vec<_>, _>>()?;
            let ret_sym = c.symbol();
            let mut ret = vec![ Instr::LoadValue(ret_sym, RuntimeData::List(vec![])) ];
            //ret.push( Instr::)
            Ok((ret_sym, ret))
        },
        Lit::Tuple(x) => {
            Err(CompileError::Todo)
        },
        Lit::Lambda(x) => {
            Err(CompileError::Todo)
        },
    }

}
