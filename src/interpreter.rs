use crate::ast::{*,Value};
use std::collections::HashMap;

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

// Evaluates a complete binomial tree to a single integer or bool.
pub fn eval_tree(e: Expr) -> Result<Value, EvalErr> {
    match e {
        Expr::Num(num) => Ok(Value::Num(num)),
        Expr::Bool(b) => Ok(Value::Bool(b)),
        Expr::Var(s) => Ok(Value::Var(s)),
        Expr::BinOp(left, op, right) => {
            eval_bin_expr(eval_tree(*left)?, op, eval_tree(*right)?)
        },
        _ => panic!(),
    }
}

pub fn eval_keyword_tree(e: Expr) {

}

pub fn test_eval() {
}
