use std::fmt;

pub type Args = Block;
pub type Params = Vec<Param>;
pub type Functions = Vec<Function>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub content: Vec<Expr>,
}

impl Block {
    pub fn new(exprs: Vec<Expr>) -> Block {
        Block { content: exprs }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathToken {
    Minus,
    Plus,
    Multiply,
    Division,
}

impl fmt::Display for MathToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            Self::Minus => "-",
            Self::Plus => "+",
            Self::Multiply => "*",
            Self::Division => "/",
        };
        write!(f, "{}", token)
    }
}

// Need to handle Not
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolToken {
    And,
    Or,
}

impl fmt::Display for BoolToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            Self::And => "&&",
            Self::Or => "||",
        };
        write!(f, "{}", token)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelToken {
    Le,
    Ge,
    Equal,
    Neq,
}

impl fmt::Display for RelToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            RelToken::Le => "<",
            RelToken::Ge => ">",
            RelToken::Equal => "==",
            RelToken::Neq => "!=",
        };
        write!(f, "{}", token)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarToken {
    Assign,
    PlusEq,
    MinEq,
    MulEq,
}

impl fmt::Display for VarToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = match self {
            VarToken::Assign => "=",
            VarToken::PlusEq => "+=",
            VarToken::MinEq => "-=",
            VarToken::MulEq => "*=",
        };
        write!(f, "{}", token)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    MathOp(MathToken),
    BoolOp(BoolToken),
    RelOp(RelToken),
    VarOp(VarToken),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::MathOp(token) => token.fmt(f),
            Op::BoolOp(token) => token.fmt(f),
            Op::RelOp(token) => token.fmt(f),
            Op::VarOp(token) => token.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Int32,
    Bool,
    Void, // for functions
}

impl From<Type> for String {
    fn from(t: Type) -> String {
        match t {
            Type::Int32 => "Int32".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Void => "Void".to_string(),
            _ => panic!("Could not convert to String. No such type found."),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub param_type: Type,
}

impl Param {
    pub fn new(name: String, param_type: Type) -> Param {
        Param { name, param_type }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub params: Params,
    pub block: Block,
    pub return_type: Type,
}

impl Function {
    pub fn new(name: String, params: Params, block: Block, return_type: Type) -> Self {
        Function {
            name,
            params,
            block,
            return_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Args,
}

impl FunctionCall {
    pub fn new(name: String, args: Args) -> Self {
        FunctionCall { name, args }
    }
}

// Value, Keyword, _Expr (rename to Expr), Node (same functionality
// as the old Expr) should be used instead of only Expr
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Value {
    Num(i32),
    Var(String),
    Bool(bool),
    Return(Box<Self>),
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

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Let(Box<Node>, Type, Box<Node>),
    If(Box<Node>, Block),
    IfElse(Box<Node>, Block), // not implemented yet
    While(Box<Node>, Block),
    FuncCall(FunctionCall),
    Return(Box<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum _Expr {
    BinOp(Box<Node>, Op, Box<Node>),
    VarOp(Box<Node>, Op, Box<Node>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Value(Value),
    Keyword(Keyword),
    Expr(_Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    // Right-hand expressions
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(i32),
    Var(String),
    Bool(bool),

    // Keywords (coud be moved to another enum?)
    Let(Box<Expr>, Type, Box<Expr>),
    VarOp(Box<Expr>, Op, Box<Expr>),
    If(Box<Expr>, Block),
    IfElse(Box<Expr>, Block), // not implemented yet
    While(Box<Expr>, Block),
    FuncCall(FunctionCall),
    Return(Box<Expr>),
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
        println!("!!!!!!!!!!!!! {:#?}", e);
        match e {
            Expr::Var(s) => s,
            Expr::Bool(b) => format!("{}", b),
            Expr::Num(i) => format!("{}", i),
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
