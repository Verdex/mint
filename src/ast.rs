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
