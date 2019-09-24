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
    Leq,
    Geq,
    Equal,
    Neq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    MathOp(MathToken),
    BoolOp(BoolToken),
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
    literal_type: Type,
}

impl Param {
    pub fn new(name: String, literal_type: Type) -> Param {
        Param { name, literal_type }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    name: String,
    params: Vec<Param>,
    //block: Vec<Expr>,
    return_type: Type,
}

impl Function {
    //pub fn new(name: String, params: Vec<Param>, block: Vec<Expr>, return_type: Type) -> Function {
    pub fn new(name: String, params: Vec<Param>, return_type: Type) -> Function {
        Function {
            name,
            params,
            //block,
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
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(i32),                        // value enum?
    Var(String),                     // value
    Bool(bool),                      // value
    Let(Box<Expr>, Type, Box<Expr>), // shold be moved to another enum

    If(Box<Expr>, Vec<Expr>), // should be moved to another enum
    Else(Vec<Expr>),
    While,
    For,

    Func(Function),
}

impl Into<Option<i32>> for Expr {
    fn into(self) -> Option<i32> {
        match self {
            Expr::Num(i) => Some(i),
            _ => None,
        }
    }
}

impl Into<Option<String>> for Expr {
    fn into(self) -> Option<String> {
        match self {
            Expr::Var(s) => Some(s),
            _ => None,
        }
    }
}

impl Expr {
    pub fn into_id(self) -> Option<String> {
        match self {
            Expr::Var(s) => Some(s),
            _ => None,
        }
    }
}
