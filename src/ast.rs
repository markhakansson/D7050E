#[derive(PartialEq, Debug, Eq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(name: &str) -> Identifier {
        Identifier(name.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MathOp {
    Minus,
    Plus,
    Multiply,
    Division,
    Modulo,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoolOp {
    And,
    Or,
    Not,
    Leq,
    Geq,
    Equal,
    Neq,
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

#[derive(Debug)]
pub enum Expr {
    BinOp(Box<Expr>, MathOp, Box<Expr>),
    LogOp(Box<Expr>, BoolOp, Box<Expr>),
    Num(i32),
    Var(Identifier),
    Bool(BoolState),
    Let(Identifier, Type, Box<Expr>),
    If(Box<Expr>,Vec<Expr>)
}
