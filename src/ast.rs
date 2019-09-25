#[derive(PartialEq, Debug, Eq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(name: &str) -> Identifier {
        Identifier(name.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MathToken {
    Minus,
    Plus,
    Multiply,
    Division,
    Modulo,
}

// Need to handle Not
#[derive(Debug, PartialEq, Eq)]
pub enum BoolToken {
    And,
    Or,
    Not,
    /*     Leq,
    Geq,
    Equal,
    Neq, */
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelToken {
    Leq,
    Geq,
    Equal,
    Neq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    MathOp(MathToken),
    BoolOp(BoolToken),
    RelOp(RelToken),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoolState {
    True,
    False,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Int32,
    Bool,
    Void, // for functions
}

#[derive(Debug, PartialEq, Eq)]
pub struct Param {
    name: String,
    param_type: Type,
}

impl Param {
    pub fn new(name: String, param_type: Type) -> Param {
        Param { name, param_type }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    name: String,
    params: Vec<Param>,
    block: Vec<Expr>,
    return_type: Type,
}

impl Function {
    pub fn new(name: String, params: Vec<Param>, block: Vec<Expr>, return_type: Type) -> Function {
        Function {
            name,
            params,
            block,
            return_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Num(i32),
    Var(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    // Right-hand expressions
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(i32),    // value enum?
    Var(String), // value
    Bool(bool),  // value

    // Keywords
    Let(Box<Expr>, Type, Box<Expr>), // shold be moved to another enum
    If(Box<Expr>, Vec<Expr>),        // should be moved to another enum
    IfElse(Box<Expr>, Vec<Expr>),
    While(Box<Expr>, Vec<Expr>),
    Func(Function),
    Return(Box<Expr>),
}

impl From<Expr> for i32 {
    fn from(e: Expr) -> i32 {
        match e {
            Expr::Num(i) => i,
            _ => panic!(),
        }
    }
}

impl From<Expr> for String {
    fn from(e: Expr) -> String {
        match e {
            Expr::Var(s) => s,
            _ => panic!(),
        }
    }
}

impl From<Expr> for bool {
    fn from(e: Expr) -> bool {
        match e {
            Expr::Bool(b) => b,
            _ => panic!(),
        }
    }
}


