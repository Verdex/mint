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
}

#[derive(Debug, Clone)]
pub enum Data {
    Number(f64),
    String(String),
    Symbol(String),
    Variable(String),
    List(Vec<Data>),
    Tuple(Vec<Data>),
    // lambda
}

#[derive(Debug)]
pub enum Pat {
    Wild,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Pat>, Option<Box<Pat>>),
    Tuple(Vec<Pat>),
    Variable(String),
    At(String, Box<Pat>),
    If(Box<Pat>, Expr),
}

#[derive(Debug)]
pub enum Expr {
    Data(Data),
    Call,
    // match
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