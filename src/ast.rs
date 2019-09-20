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
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arg {
    name: String,
    literal_type: Type,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Function {
    name: String,
    arguments: Vec<Arg>,
    block: Vec<Expr>,
    return_type: Type,
}

impl Function {
    pub fn new(name: String, arguments: Vec<Arg>, block: Vec<Expr>, return_type: Type) -> Function {
        Function {
            name,
            arguments,
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
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(i32),    // value enum?
    Var(String), // value
    Bool(bool),  // value

    Let(Box<Expr>, Type, Box<Expr>), // shold be moved to another enum
    If(Box<Expr>, Vec<Expr>),        // should be moved to another enum
    Else(Vec<Expr>),

    Func(Function),
}
