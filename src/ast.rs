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
}

#[derive(Debug)]
pub enum Data {
    Number(f64),
    String(String),
    Symbol(String),
    Variable(String),
    // list
    // tuple
    // lambda
}

#[derive(Debug)]
pub enum Pat {
    Nil
}

#[derive(Debug)]
pub enum Expr {
    Data(Data),
    Call,
}

#[derive(Debug)]
pub struct Let {
    pub pattern : Pat,
    pub expr : Expr,
}

#[derive(Debug)]
pub struct Top {
    pub lets : Vec<Let>,
    pub expr : Expr,
}