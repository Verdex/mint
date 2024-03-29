use std::collections::HashMap;

use purple::data::*;
use denest::Linearizable;

use crate::runtime::*;
use crate::ast::*;

use super::error::*;
use super::instr;


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
    fn single( s : Symbol, i : Instr<RuntimeData, Heap> ) -> Result<(Symbol, Vec<I>), StaticError> {
        Ok((s, vec![ i ]))
    }

    match input {
        Lit::Number(x) => { 
            let s = c.symbol();
            single(s, Instr::LoadValue(s, RuntimeData::Number(*x)))
        },
        Lit::String(x) => {
            let s = c.symbol();
            single(s, Instr::LoadValue(s, RuntimeData::String(x.to_string())))
        },
        Lit::Symbol(x) => {
            let s = c.symbol();
            single(s, Instr::LoadValue(s, RuntimeData::Symbol(x.to_string())))
        },
        Lit::Variable(x) if !address_map.contains_key(x) => Err(StaticError::VariableNotDefined(x.into())), 
        Lit::Variable(x) => {
            let address = address_map.get(x).unwrap().clone();
            let s = c.symbol();
            single(s, instr::load_from_heap(address, s))
        }, 
        Lit::List(x) => {
            let y = x.iter().map(|d| compile_literal(c, d, address_map, functions)).collect::<Result<Vec<_>, _>>()?;
            let ret_sym = c.symbol();
            let ret_address = c.symbol();
            let mut ret : Vec<I> = vec![ Instr::LoadValue(ret_sym, RuntimeData::List(vec![])) ];

            ret.push(instr::insert_into_heap(ret_sym, ret_address));

            let (item_names, progs) : (Vec<_>, Vec<_>) = y.into_iter().unzip(); // TODO is this possible ahead of time?
            let mut progs = progs.into_iter().flatten().collect::<Vec<_>>();

            ret.append(&mut progs);

            let mut item_names : Vec<I> = item_names.into_iter()
                                                    .map(|item| instr::push_into_list_in_heap(item, ret_address))
                                                    .collect();

            ret.append(&mut item_names);

            Ok((ret_address, ret))
        },
        Lit::Tuple(x) => {
            let y = x.iter().map(|d| compile_literal(c, d, address_map, functions)).collect::<Result<Vec<_>, _>>()?;
            let ret_sym = c.symbol();
            let ret_address = c.symbol();
            let mut ret : Vec<I> = vec![ Instr::LoadValue(ret_sym, RuntimeData::Tuple(vec![])) ];

            ret.push(instr::insert_into_heap(ret_sym, ret_address));

            let (item_names, progs) : (Vec<_>, Vec<_>) = y.into_iter().unzip(); // TODO is this possible ahead of time?
            let mut progs = progs.into_iter().flatten().collect::<Vec<_>>();

            ret.append(&mut progs);

            let mut item_names : Vec<I> = item_names.into_iter()
                                                    .map(|item| instr::push_into_tuple_in_heap(item, ret_address))
                                                    .collect();

            ret.append(&mut item_names);

            Ok((ret_address, ret))
        },
        Lit::Lambda(x) => {
            lambda_variables_are_unique(&x)?;
            
            let mut func_body : Vec<I> = vec![];

            let mut var_to_sym : HashMap<String, Symbol> = HashMap::new();

            // TODO use this map to figure out what symbol a variable needs to grab
            /*let mut var_to_sym = x.params.iter().flat_map(|param| param.variables_to_bind())
                                                .map(|var| (var, c.symbol()))
                                                .collect::<Vec<_>>()
                                                .into_iter()
                                                .chain(x.body.variables_to_bind().map(|var| (var, c.symbol())))
                                                .collect::<HashMap<&str, Symbol>>();*/

            let mut pre_datas = vec![];
            for _ in 0..x.params.len() {
                let s = c.symbol();
                pre_datas.push(s);
                func_body.push(Instr::PopParam(s));
            }
            pre_datas.reverse();

            for (param, pre_data) in std::iter::zip(&x.params, pre_datas) {
                let result = c.symbol();
                let local_var_to_sym = param.variables_to_bind().map(|var| (var.to_string(), c.symbol())).collect::<HashMap<String, Symbol>>();
                func_body.push(instr::pattern_match(pre_data, param.clone(), result, local_var_to_sym.clone()));
                func_body.push(instr::panic_on_false(result, "".into()));
                for (k, v) in local_var_to_sym.into_iter() {
                    var_to_sym.insert(k, v);
                }
            }

            let func_address = c.func();

            functions.insert(func_address, func_body);

            let func_address_sym = c.symbol();

            // TODO compile body

            single(func_address_sym, Instr::LoadFunc(func_address_sym, func_address))
        },
    }
}

fn lambda_variables_are_unique( lambda : &Lambda ) -> Result<(), StaticError> {
    let mut variables_to_bind = lambda.body.variables_to_bind()
        .chain(lambda.params.iter()
                            .flat_map(|x| x.variables_to_bind()))
        .collect::<Vec<_>>();
    
    variables_to_bind.sort();

    for (a, b) in std::iter::zip(variables_to_bind.iter(), variables_to_bind.iter().skip(1)) {
        if a == b {
            return Err(StaticError::DuplicateVariableDefinitions((**a).into()));
        }
    }

    Ok(())
}

#[cfg(test)] 
mod test {
}