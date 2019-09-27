use crate::ast::{*,Value::{Num,Bool,Var}};
use std::collections::HashMap;

type Scope = HashMap<Value, Value>;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalErr {
    WrongOp(String),
    TypeMismatch(String),
}

fn eval_i32_expr(l: i32, op: Op, r: i32) -> Result<Value, EvalErr> {
    match op {
        Op::MathOp(MathToken::Division) => Ok(Value::Num(l / r)),
        Op::MathOp(MathToken::Multiply) => Ok(Value::Num(l * r)),
        Op::MathOp(MathToken::Plus) => Ok(Value::Num(l + r)),
        Op::MathOp(MathToken::Minus) => Ok(Value::Num(l - r)),
        Op::MathOp(MathToken::Modulo) => Ok(Value::Num(l % r)),
        Op::RelOp(RelToken::Equal) => Ok(Value::Bool(l == r)),
        Op::RelOp(RelToken::Geq) => Ok(Value::Bool(l > r)),
        Op::RelOp(RelToken::Leq) => Ok(Value::Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Value::Bool(l != r)),
        _ => Err(EvalErr::WrongOp(String::from("Not an i32 operator."))),
    }
}

fn eval_bool_expr(l: bool, op: Op, r: bool) -> Result<Value, EvalErr> {
    match op {
        Op::BoolOp(BoolToken::And) => Ok(Value::Bool(l && r)),
        Op::BoolOp(BoolToken::Or) => Ok(Value::Bool(l || r)),
        Op::RelOp(RelToken::Equal) => Ok(Value::Bool(l == r)),
        Op::RelOp(RelToken::Geq) => Ok(Value::Bool(l > r)),
        Op::RelOp(RelToken::Leq) => Ok(Value::Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Value::Bool(l != r)),
        _ => Err(EvalErr::WrongOp(String::from("Not a boolean operator."))),
    }
}

// Evaluates whether an expression is an i32 or bool operation.
fn eval_bin_expr(l: Value, op: Op, r: Value) -> Result<Value, EvalErr> {
    match (&l, &r) {
        (Value::Num(l), Value::Num(r)) => eval_i32_expr(*l, op, *r),
        (Value::Bool(l), Value::Bool(r)) => eval_bool_expr(*l, op, *r),
        _ => Err(EvalErr::TypeMismatch(String::from(
            "Can not evaluate an operation between a bool and an i32.",
        ))),
    }
}

pub fn eval_assign_var(var: Expr, var_ty: Type, expr: Expr) {

}

// Evaluates a complete binomial tree to a single integer or bool.
pub fn eval_tree(e: Expr, map: &mut Scope) -> Result<(Value, &Scope), EvalErr> {
    match e {
        Expr::Num(num) => Ok((Num(num), map)),
        Expr::Bool(b) => Ok((Bool(b), map)),
        Expr::Var(s) => Ok((Var(s), map)),
        Expr::BinOp(left, op, right) => {
            let (l_val, _) = eval_tree(*left, map)?;
            let (r_val, _) = eval_tree(*right, map)?;
            Ok((eval_bin_expr(l_val, op, r_val)?, map))
        },
        Expr::Let(var, var_ty, expr) => {
            let mut id = Var(String::from(*var));
            let (exp_val,_) = eval_tree(*expr, map)?; 
            map.insert(id, exp_val.clone());
            Ok( (exp_val, map) )
        },
        _ => panic!(),
    }
}

pub fn eval_keyword_tree(e: Expr) {

}

pub fn test_eval(e: Expr) {
    let mut map: HashMap<Value,Value> = HashMap::new();
    let (val,hash) = eval_tree(e, &mut map).unwrap();
    println!("Val: {:#?}, HashMap: {:#?}",val,hash);
}

