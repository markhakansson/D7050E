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
    Not, // implementation neeeded
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelToken {
    Leq,
    Geq,
    Equal,
    Neq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VarToken {
    Assign,
    PlusEq,
    MinEq,
    MulEq,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    MathOp(MathToken),
    BoolOp(BoolToken),
    RelOp(RelToken),
    VarOp(VarToken),
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

// Value, Keyword, NewExpr (rename to Expr), Node (same functionality
// as the old Expr) should be used instead of only Expr
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Value {
    Num(i32),
    Var(String),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Let(Box<Expr>, Type, Box<Expr>),
    If(Box<Expr>, Vec<Expr>),
    IfElse(Box<Expr>, Vec<Expr>),
    While(Box<Expr>, Vec<Expr>),
    Func(Function),
    Return(Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum NewExpr {
    BinOp(Box<Node>, Op, Box<Node>),
    VarOp(Box<Expr>, Op, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Value(Value),
    Keyword(Keyword),
    Expr(NewExpr),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    // Right-hand expressions
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(i32),
    Var(String),
    Bool(bool),

    // Keywords (coud be moved to another enum?)
    Let(Box<Expr>, Type, Box<Expr>),
    VarOp(Box<Expr>, Op, Box<Expr>),
    If(Box<Expr>, Vec<Expr>),
    IfElse(Box<Expr>, Vec<Expr>),
    While(Box<Expr>, Vec<Expr>),
    Func(Function),
    Return(Box<Expr>),
}

impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Num(i)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Expr> for i32 {
    fn from(e: Expr) -> i32 {
        match e {
            Expr::Num(i) => i,
            _ => panic!("Could not convert to i32. Wrong type."),
        }
    }
}

// Implement TryForm trait instead to get a Result back
impl From<Expr> for String {
    fn from(e: Expr) -> String {
        match e {
            Expr::Var(s) => s,
            _ => panic!("Could not convert to String. Wrong type."),
        }
    }
}

// Implement TryForm trait instead to get a Result back
impl From<Expr> for bool {
    fn from(e: Expr) -> bool {
        match e {
            Expr::Bool(b) => b,
            _ => panic!("Could not convert to bool. Wrong type."),
        }
    }
}

// Implement TryForm trait instead to get a Result back
impl From<Value> for i32 {
    fn from(v: Value) -> i32 {
        match v {
            Value::Num(i) => i,
            _ => panic!("Could not convert to i32. Wrong type."),
        }
    }
}

// Implement TryForm trait instead to get a Result back
impl From<Value> for bool {
    fn from(v: Value) -> bool {
        match v {
            Value::Bool(b) => b,
            _ => panic!("Could not convert bool. Wrong type."),
        }
    }
}