use crate::ast::{*,Value::{Num,Bool,Var}};
use std::collections::HashMap;

type Scope = HashMap<Value, Value>;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalErr {
    WrongOp(String),
    TypeMismatch(String),
    NotFound(String),
}

fn eval_i32_expr(l: i32, op: Op, r: i32) -> Result<Value, EvalErr> {
    match op {
        Op::MathOp(MathToken::Division) => Ok(Num(l / r)),
        Op::MathOp(MathToken::Multiply) => Ok(Num(l * r)),
        Op::MathOp(MathToken::Plus) => Ok(Num(l + r)),
        Op::MathOp(MathToken::Minus) => Ok(Num(l - r)),
        Op::MathOp(MathToken::Modulo) => Ok(Num(l % r)),
        Op::RelOp(RelToken::Equal) => Ok(Bool(l == r)),
        Op::RelOp(RelToken::Geq) => Ok(Bool(l > r)),
        Op::RelOp(RelToken::Leq) => Ok(Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Bool(l != r)),
        _ => Err(EvalErr::WrongOp(String::from("Not an i32 operator."))),
    }
}

fn eval_bool_expr(l: bool, op: Op, r: bool) -> Result<Value, EvalErr> {
    match op {
        Op::BoolOp(BoolToken::And) => Ok(Bool(l && r)),
        Op::BoolOp(BoolToken::Or) => Ok(Bool(l || r)),
        Op::RelOp(RelToken::Equal) => Ok(Bool(l == r)),
        Op::RelOp(RelToken::Geq) => Ok(Bool(l > r)),
        Op::RelOp(RelToken::Leq) => Ok(Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Bool(l != r)),
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

/* pub fn eval_assign_var(var: Expr, var_ty: Type, expr: Expr, map: &mut Scope) -> Result<(Value, &Scope), EvalErr> {
    let mut id = Var(String::from(var));
    let (expr_val,_) = eval_tree(expr, map)?; 
    map.insert(id, expr_val.clone());
    Ok( (expr_val, map) ) 
} */

// Evaluates a complete binomial tree to a single integer or bool.
pub fn eval_expr(e: Expr, map: &mut Scope) -> Result<Value, EvalErr> {
    match e {
        Expr::Num(num) => Ok(Num(num)),
        Expr::Bool(b) => Ok(Bool(b)),
        Expr::Var(s) => {
            Ok(get_val_from_map(Var(s),map)?)
        },
        Expr::BinOp(left, op, right) => {
            let l_val = eval_expr(*left, map)?;
            let r_val = eval_expr(*right, map)?;
            Ok(eval_bin_expr(l_val, op, r_val)?)
        },
        Expr::Let(var, var_ty, expr) => { // not sure how to deal with type. ignore it for now
            let id = Var(String::from(*var));
            let expr_val = eval_expr(*expr, map)?; 
            map.insert(id, expr_val.clone());
            Ok(expr_val)
        },
        _ => panic!(),
    }
}

// Gets the value for a variable in the hashmap
fn get_val_from_map(key: Value, map: &mut Scope) -> Result<Value,EvalErr> {
    match map.get(&key) {
        Some(value) => Ok(value.clone()),
        None => Err(EvalErr::NotFound("Key not found in hashmap.".to_string())),
    }
}



pub fn test_eval(e: Expr) {
    let mut scope: Scope = HashMap::new();
    //let mut context: Vec<Scope> = vec![];
    scope.insert(Var("a".to_string()), Num(32 as i32));
    //context.push(scope);
    let val= eval_expr(e, &mut scope).unwrap();
    println!("Val: {:#?}, HashMap: {:#?}", val, scope);
}

