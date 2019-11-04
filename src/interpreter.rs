use crate::ast::{
    Value::{Bool, Num},
    *,
};
use crate::context;
use crate::context::*;
use std::collections::HashMap;

pub type EvalRes<T> = Result<T, EvalErr>;
pub type FnContext<'a> = context::ContextStack<'a, Value>;
pub type Context = context::Context<Value>;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalErr {
    NotFound(String),
    NotImplemented,
    TypeMismatch(String),
    WrongOp(String),
    WrongType(String),
}

fn eval_i32_expr(l: i32, op: Op, r: i32) -> EvalRes<Value> {
    match op {
        Op::MathOp(MathToken::Division) => Ok(Num(l / r)),
        Op::MathOp(MathToken::Multiply) => Ok(Num(l * r)),
        Op::MathOp(MathToken::Plus) => Ok(Num(l + r)),
        Op::MathOp(MathToken::Minus) => Ok(Num(l - r)),
        Op::RelOp(RelToken::Equal) => Ok(Bool(l == r)),
        Op::RelOp(RelToken::Ge) => Ok(Bool(l > r)),
        Op::RelOp(RelToken::Le) => Ok(Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Bool(l != r)),
        _ => Err(EvalErr::WrongOp(String::from("Not an i32 operator."))),
    }
}

fn eval_bool_expr(l: bool, op: Op, r: bool) -> EvalRes<Value> {
    match op {
        Op::BoolOp(BoolToken::And) => Ok(Bool(l && r)),
        Op::BoolOp(BoolToken::Or) => Ok(Bool(l || r)),
        Op::RelOp(RelToken::Equal) => Ok(Bool(l == r)),
        Op::RelOp(RelToken::Ge) => Ok(Bool(l > r)),
        Op::RelOp(RelToken::Le) => Ok(Bool(l < r)),
        Op::RelOp(RelToken::Neq) => Ok(Bool(l != r)),
        _ => Err(EvalErr::WrongOp(String::from("Not a boolean operator."))),
    }
}

// Evaluates whether an expression is an i32 or bool operation.
fn eval_bin_expr(
    l: Expr,
    op: Op,
    r: Expr,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    let l_val = eval_expr(l, fn_tree, fn_context)?;
    let r_val = eval_expr(r, fn_tree, fn_context)?;

    match (l_val, r_val) {
        (Num(l_val), Num(r_val)) => eval_i32_expr(l_val, op, r_val),
        (Bool(l_val), Bool(r_val)) => eval_bool_expr(l_val, op, r_val),
        _ => Err(EvalErr::TypeMismatch(String::from(
            "Can not evaluate an operation between a bool and an i32.",
        ))),
    }
}

// Evaluates a complete binomial tree to a single integer or bool.
// Should clone the expression before first calling the function. Except
// in functions that eval_expr itself calls.
pub fn eval_expr(e: Expr, fn_tree: &mut Functions, fn_context: &mut FnContext) -> EvalRes<Value> {
    //let context = fn_context.get_last_context()?;
    match e {
        Expr::Num(num) => Ok(Num(num)),
        Expr::Bool(b) => Ok(Bool(b)),
        Expr::Var(s) => fn_context.get_last_context()?.get_val(&s),
        Expr::BinOp(left, op, right) => eval_bin_expr(*left, op, *right, fn_tree, fn_context),
        Expr::VarOp(var, op, expr) => {
            let key = String::from(*var);
            let expr_val = eval_expr(*expr, fn_tree, fn_context)?;

            match op {
                Op::VarOp(VarToken::Assign) => {
                    fn_context.get_last_context()?.update_var(&key, &expr_val)
                }
                _ => eval_var_op(&key, op, &expr_val, fn_context.get_last_context()?),
            }
        }
        Expr::Let(var, _, expr) => assign_var(*var, *expr, fn_tree, fn_context), // ignore type for now
        Expr::If(expr, block) => eval_if(*expr, block, fn_tree, fn_context),
        Expr::FuncCall(fn_call) => eval_fn_call(fn_call, fn_tree, fn_context),
        Expr::Return(val) => Ok(Value::Return(Box::new(eval_expr(
            *val, fn_tree, fn_context,
        )?))),
        Expr::While(expr, block) => eval_while(*expr, block, fn_tree, fn_context),
        _ => Err(EvalErr::NotImplemented),
    }
}

// Assigns value to variable. Store it in current scope.
fn assign_var(
    var: Expr,
    expr: Expr,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    let id = String::from(var);
    let expr_val = eval_expr(expr, fn_tree, fn_context)?;
    fn_context
        .get_last_context()?
        .insert_to_current_scope(&id, &expr_val);
    Ok(expr_val)
}

// Evaluates variable operations such as ´a += b´ etc.
fn eval_var_op(key: &str, op: Op, new_val: &Value, context: &mut Context) -> EvalRes<Value> {
    let old_val: i32 = i32::from(context.get_val(&key)?);
    let expr_val: i32 = i32::from(new_val.clone());

    match op {
        Op::VarOp(VarToken::PlusEq) => {
            let new_val = Num(old_val + expr_val);
            context.update_var(&key, &new_val)
        }
        Op::VarOp(VarToken::MinEq) => {
            let new_val = Num(old_val - expr_val);
            context.update_var(&key, &new_val)
        }
        Op::VarOp(VarToken::MulEq) => {
            let new_val = Num(old_val * expr_val);
            context.update_var(&key, &new_val)
        }
        _ => Err(EvalErr::WrongOp("Not a variable operator.".to_string())),
    }
}

fn eval_if(
    e: Expr,
    block: Block,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    let condition = eval_expr(e, fn_tree, fn_context)?;
    let res: EvalRes<Value>;

    match condition {
        Bool(true) => {
            res = eval_block(block, fn_tree, fn_context);
        }
        Bool(false) => res = Ok(Bool(false)),
        _ => {
            res = Err(EvalErr::WrongType(
                "Cannot evaluate condition. Not a boolean expression.".to_string(),
            ))
        }
    }

    res
}

pub fn eval_while(
    e: Expr,
    block: Block,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    let if_val: EvalRes<Value> = eval_if(e.clone(), block.clone(), fn_tree, fn_context);
    let ret_val: EvalRes<Value>;
    println!("while");
    match if_val {
        Ok(Bool(false)) => ret_val = if_val,
        Ok(_) => ret_val = eval_while(e.clone(), block.clone(), fn_tree, fn_context),
        Err(_) => {
            ret_val = Err(EvalErr::WrongType(
                "Cannot evaluate condition. Not a boolean expression.".to_string(),
            ))
        }
    }

    ret_val
}

// Evaluates a complete block. Returns the value from the last instruction evaluated.
pub fn eval_block(
    block: Block,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    fn_context.get_last_context()?.new_scope();

    let mut res: EvalRes<Value> = Err(EvalErr::NotFound("No expressions found.".to_string()));
    for e in block.content.iter() {
        res = eval_expr(e.clone(), fn_tree, fn_context);
        match res {
            Ok(Value::Return(_)) => break,
            _ => continue,
        }
    }

    // Drop scope (comment out for debug)
    fn_context.get_last_context()?.drop_current_scope();

    res
}

pub fn eval_fn_call(
    fn_call: FunctionCall,
    fn_tree: &mut Functions,
    fn_context: &mut FnContext,
) -> EvalRes<Value> {
    println!("{:#?}", fn_context);
    // Get the argument values.
    let mut arg_values: Vec<Value> = Vec::new();
    for arg in fn_call.args.content {
        arg_values.push(eval_expr(arg, fn_tree, fn_context)?);
    }

    // Match the argument values with the parameter names. Place into the top scope of a new context.
    let func_temp: EvalRes<Function> = fn_tree.get_fn(fn_call.name);
    let func = func_temp?;
    fn_context.new_context()?.new_scope();
    let context = fn_context.get_last_context()?;
    let mut step = 0;
    for param in func.params {
        context.insert_to_current_scope(&param.name, &arg_values[step]);
        step += 1;
    }

    let return_val = eval_block(func.block, fn_tree, fn_context);

    // Drop the function's context (comment out for debug)
    fn_context.drop_current_context();

    // Unwraps return statements
    match return_val {
        Err(e) => Err(e),
        Ok(Value::Return(val)) => Ok(*val),
        _ => return_val,
    }
}

// Main entry
pub fn eval_program(fn_tree: &mut Functions) -> EvalRes<Value> {
    // Setup new contexts
    let mut fn_context = FnContext::new();
    fn_context.new_context()?;

    let main_res: EvalRes<Function> = fn_tree.get_fn("main".to_string());
    let main = main_res?;

    eval_block(main.block, fn_tree, &mut fn_context)
}

// Returns the FnContext instead
pub fn eval_program_debug(fn_tree: &mut Functions) -> EvalRes<FnContext> {
    // Setup new contexts
    let mut fn_context = FnContext::new();
    fn_context.new_context()?;

    let main_res: EvalRes<Function> = fn_tree.get_fn("main".to_string());
    let main: Function = main_res?;

    eval_block(main.block, fn_tree, &mut fn_context)?;

    Ok(fn_context)
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use crate::parser::*;

    #[test]
    fn eval_fibo_program() {
        let fibo = "
            fn fibo(i: i32) -> i32 {
                if i == 1 {
                    return 0;
                };
                if i == 2 {
                    return 1;
                };

                return fibo(i-1) + fibo(i-2);

            }

            fn fuck(off: i32) -> i32 {
                return off*2;
            }

            fn main() -> () {
                let a: i32 = 7;
                fibo(a);
            }

        ";

        let mut tree = parse_program(fibo).unwrap().1;
        assert_eq!(eval_program(&mut tree).is_ok(), true);
        assert_eq!(eval_program(&mut tree).unwrap(), Num(8));
    }

    #[test]
    fn test_eval_program() {
        let main = "
            fn test(i: i32) -> () {
                let a: i32 = 3 + i;
            }

            fn main() -> () {
                let a: i32 = 300;
                let b: bool = true;
                let c: i32 = 0;
                while b {
                    c = 1;
                    b = false;
                };
                test(a);
            }";
        let mut tree = parse_program(main).unwrap().1;
        assert_eq!(eval_program(&mut tree).is_ok(), true);
    }
}
