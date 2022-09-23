use std::collections::HashMap;

use purple::data::*;
use crate::runtime::*;
use crate::ast::*;

use super::error::*;


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

pub fn compile(input : &Expr, address_map : &M, functions : &mut Fs) -> Result<Vec<I>, StaticError> {
    let mut func = functions.keys().map(|k| k.0).max().unwrap_or(0);
    let mut c = C::new(func);
    match input {
        Expr::Literal(lit) => {
            let (sym, mut prog) = compile_literal(&mut c, &lit, address_map, functions)?;
            let mut x = vec![ Instr::Return(sym) ];
            prog.append(&mut x);
            Ok(prog)
        },
        Expr::Call(func_expr, params) => Err(StaticError::Todo),
    }
}

fn compile_literal(c : &mut C, input : &Lit, address_map : &M, functions : &mut Fs) -> Result<(Symbol, Vec<I>), StaticError> {
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
            Err(StaticError::Todo)
        },
        Lit::List(x) => {
            let y = x.iter().map(|d| compile_literal(c, d, address_map, functions)).collect::<Result<Vec<_>, _>>()?;
            let ret_sym = c.symbol();
            let ret_address = c.symbol();
            let mut ret : Vec<I> = vec![ Instr::LoadValue(ret_sym, RuntimeData::List(vec![])) ];
            ret.push( Instr::LoadFromSysCall(ret_address, Box::new(
                move |locals, heap| {
                    if let Data::Value(list) = locals.get(&ret_sym)? {
                        let address = heap.insert_new(list);
                        Ok(Data::Value(RuntimeData::Address(address)))
                    }
                    else {
                        Err(Box::new(DynamicError::TypeMismatch { 
                            expected: "Data::Value".into(),
                            observed: "?".into(),
                        }))
                    }
                }
            )));

            let (item_names, progs) : (Vec<_>, Vec<_>) = y.into_iter().unzip(); // TODO is this possible ahead of time?
            let mut progs = progs.into_iter().flatten().collect::<Vec<_>>();

            ret.append(&mut progs);

            let mut item_names : Vec<I> = item_names.into_iter().map(|n| Instr::<RuntimeData, Heap>::SysCall(Box::new(
                move |locals, heap| {
                    if let (Data::Value(target), Data::Value(RuntimeData::Address(list_address))) 
                        = (locals.get(&n)?, locals.get(&ret_address)?) {

                        let list = heap.get_mut(list_address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?;

                        if let RuntimeData::List(l) = list {
                            l.push(target);
                        }
                        else {
                            return Err(Box::new(DynamicError::TypeMismatch {
                                expected: "RuntimeData::List".into(),
                                observed: "?".into(),
                            }));
                        }

                        Ok(())
                    }
                    else {
                        Err(Box::new(DynamicError::TypeMismatch {
                            expected: "Data::Value and Data::Value(RuntimeData::Address)".into(),
                            observed: "?".into(),
                        }))
                    }
                }
            ))).collect();

            ret.append(&mut item_names);

            Ok((ret_address, ret))
        },
        Lit::Tuple(x) => {
            let y = x.iter().map(|d| compile_literal(c, d, address_map, functions)).collect::<Result<Vec<_>, _>>()?;
            let ret_sym = c.symbol();
            let ret_address = c.symbol();
            let mut ret : Vec<I> = vec![ Instr::LoadValue(ret_sym, RuntimeData::Tuple(vec![])) ];
            ret.push( Instr::LoadFromSysCall(ret_address, Box::new(
                move |locals, heap| {
                    if let Data::Value(list) = locals.get(&ret_sym)? {
                        let address = heap.insert_new(list);
                        Ok(Data::Value(RuntimeData::Address(address)))
                    }
                    else {
                        Err(Box::new(DynamicError::TypeMismatch { 
                            expected: "Data::Value".into(),
                            observed: "?".into(),
                        }))
                    }
                }
            )));

            let (item_names, progs) : (Vec<_>, Vec<_>) = y.into_iter().unzip(); // TODO is this possible ahead of time?
            let mut progs = progs.into_iter().flatten().collect::<Vec<_>>();

            ret.append(&mut progs);

            let mut item_names : Vec<I> = item_names.into_iter().map(|n| Instr::<RuntimeData, Heap>::SysCall(Box::new(
                move |locals, heap| {
                    if let (Data::Value(target), Data::Value(RuntimeData::Address(list_address))) 
                        = (locals.get(&n)?, locals.get(&ret_address)?) {

                        let list = heap.get_mut(list_address).ok_or(Box::new(DynamicError::CannotFindHeapAddress))?;

                        if let RuntimeData::Tuple(l) = list {
                            l.push(target);
                        }
                        else {
                            return Err(Box::new(DynamicError::TypeMismatch {
                                expected: "RuntimeData::Tuple".into(),
                                observed: "?".into(),
                            }));
                        }

                        Ok(())
                    }
                    else {
                        Err(Box::new(DynamicError::TypeMismatch {
                            expected: "Data::Value and Data::Value(RuntimeData::Address)".into(),
                            observed: "?".into(),
                        }))
                    }
                }
            ))).collect();

            ret.append(&mut item_names);

            Ok((ret_address, ret))
        },
        Lit::Lambda(x) => {
            Err(StaticError::Todo)
        },
    }
}