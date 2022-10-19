#[derive(Debug, Clone, Copy)]
pub struct TMeta {
    pub start : usize,
    pub end : usize,
}

#[derive(Debug)]
pub enum Token {
    LowerSymbol(TMeta, String),
    UpperSymbol(TMeta, String),
    Number(TMeta, f64),
    String(TMeta, String),
    LParen(TMeta),
    RParen(TMeta),
    LCurl(TMeta),
    RCurl(TMeta),
    LSquare(TMeta),
    RSquare(TMeta),
    LAngle(TMeta),
    RAngle(TMeta),
    SLArrow(TMeta),
    SRArrow(TMeta),
    DLArrow(TMeta),
    DRArrow(TMeta),
    Colon(TMeta),
    Dot(TMeta),
    Comma(TMeta),
    Equal(TMeta),
    Semicolon(TMeta),
    OrBar(TMeta),
    At(TMeta),
}

// TODO see if we can get rid of clone

#[derive(Debug, Clone)]
pub struct Lambda {
    pub params : Vec<Pat>,
    pub body : Box<Top>,
}

#[derive(Debug, Clone)]
pub enum Lit {
    Number(f64),
    String(String),
    Symbol(String),
    Variable(String),
    List(Vec<Lit>),
    Tuple(Vec<Lit>),
    Lambda(Lambda),
}

#[derive(Debug, Clone)]
pub enum Pat {
    Wild,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Pat>, Option<Box<Pat>>),
    Tuple(Vec<Pat>),
    Variable(String),
    At(String, Box<Pat>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Lit),
    Call(Box<Expr>, Vec<Expr>),
    // match
// TODO add if to match cases
}

#[derive(Debug, Clone)]
pub struct Let {
    pub pattern : Pat,
    pub expr : Expr,
}

#[derive(Debug, Clone)]
pub struct Top {
    pub lets : Vec<Let>,
    pub expr : Option<Expr>,
}

use denest::Linearizable;

impl<'a> Linearizable<'a> for Pat {
    
    fn l_next(&'a self) -> Vec<&'a Self> {
        use crate::ast::Pat::*;

        match self {
            Wild => vec![],
            Number(_) => vec![],
            String(_) => vec![],
            Symbol(_) => vec![],
            List(ps, Some(mp)) => ps.iter().chain(std::iter::once(&**mp)).collect(),
            List(ps, None) => ps.iter().collect(),
            Tuple(ps) => ps.iter().collect(),
            Variable(_) => vec![],
            At(_, p) => vec![ p ],
        }
    }
}

impl Top {
    pub fn variables_to_bind<'a>(&'a self) -> impl Iterator<Item = &'a str> {
       self.lets.iter().map(|x| &x.pattern)
                       .flat_map(|x| x.variables_to_bind())
    }
}

impl Pat { 
    pub fn variables_to_bind<'a>(&'a self) -> impl Iterator<Item = &'a str> {
        self.to_lax()
            .filter(|p| matches!(p, Pat::Variable(_) | Pat::At(_, _)))
            .map(|p| match p {
                Pat::Variable(x) => x.as_str(),
                Pat::At(x, _) => x.as_str(),
                _ => unreachable!(), 
            })
    }
}


#[cfg(test)]
mod test { 
    use super::*;

    #[test]
    fn bound_variables_should_return_all_variables_in_top_scope() {
        let tokens = crate::parsing::tokenizer::tokenize(
            "let X = 5;
             let Y = 6;
             let Z @ _  = 7;
             let { W } = 8;
             let H @ { N } = 9;
             ").unwrap();
        let top = crate::parsing::parser::parse(&tokens).unwrap();

        let output = top.variables_to_bind().collect::<Vec<_>>();

        assert_eq!( output, vec!["X", "Y", "Z", "W", "H", "N"] );
    }

    #[test]
    fn bound_variables_should_return_no_inner_scope_variables() {
        let tokens = crate::parsing::tokenizer::tokenize(
            "let X = fun(A, B, C) { let D = 8; };
             let Y = 6;
             let Z @ _  = 7;
             let { W } = 8;
             let H @ { N } = 9;
             ").unwrap();
        let top = crate::parsing::parser::parse(&tokens).unwrap();

        let output = top.variables_to_bind().collect::<Vec<_>>();

        assert_eq!( output, vec!["X", "Y", "Z", "W", "H", "N"] );
    }

}