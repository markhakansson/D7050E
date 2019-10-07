use crate::ast::{
    Value::{Bool, Num, Var},
    *,
};
use std::collections::HashMap;

pub type Scope = HashMap<String, Value>;
pub type Context = Vec<Scope>; // Context is a stack of scopes
pub type FnContext = Vec<Context>; // FnContext is a stack of scopes
pub type Functions = Vec<Expr>;

type EvalRes<T> = Result<T, EvalErr>;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalErr {
    NotFound(String),
    NotImplemented,
    TypeMismatch(String),
    WrongOp(String),
    WrongType(String),
}

pub trait ContextMethods {
    fn update_var(&mut self, key: &str, val: &Value) -> EvalRes<Value>;
    fn drop_current_scope(&mut self);
    fn get_val(&mut self, key: &str) -> EvalRes<Value>;
    fn insert_to_current_scope(&mut self, key: &str, val: &Value);
    fn new_scope(&mut self);
}

pub trait FnContextMethods {
    fn drop_current_context(&mut self);
    fn get_last_context(&mut self) -> EvalRes<&mut Context>;
    fn new_context(&mut self);
}

impl ContextMethods for Context {
    fn update_var(&mut self, key: &str, val: &Value) -> EvalRes<Value> {
        for scope in self.iter_mut().rev() {
            match scope.get(key) {
                Some(_) => {
                    scope.insert(key.to_string(), val.clone());
                    return Ok(val.clone())
                }
                None => continue,
            }
        }

        Err(EvalErr::NotFound("Value not found in context.".to_string()))
    }

    fn drop_current_scope(&mut self) {
        self.pop();
    }

    fn get_val(&mut self, key: &str) -> EvalRes<Value> {
        let mut val_res: EvalRes<Value> = Err(EvalErr::NotFound("Key not found in context scopes".to_string()));

        for scope in self.iter().rev() {
            match scope.get(key) {
                Some(value) => {
                    val_res = Ok(value.clone()); 
                    break;
                },
                None => continue,
            };
        }

        val_res
    }

    fn insert_to_current_scope(&mut self, key: &str, val: &Value) {
        let scope_opt = self.last_mut();
        match scope_opt {
            Some(scope) => scope.insert(key.to_string(), val.clone()),
            None => panic!("There are no scopes in the context."),
        };
    }
    
    fn new_scope(&mut self) {
        let scope: Scope = HashMap::new();
        self.push(scope);
    }

}

impl FnContextMethods for FnContext {
    fn drop_current_context(&mut self) {
        self.push(Context::new());
    }

    fn get_last_context(&mut self) -> EvalRes<&mut Context> {
        match self.last_mut() {
            Some(context) => Ok(context),
            None => Err(EvalErr::NotFound("No context found in FnContext.".to_string()))
        }
    }

    fn new_context(&mut self) {
        self.pop();
    }
}

fn eval_i32_expr(l: i32, op: Op, r: i32) -> EvalRes<Value> {
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

fn eval_bool_expr(l: bool, op: Op, r: bool) -> EvalRes<Value> {
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
fn eval_bin_expr(l: Expr, op: Op, r: Expr, fn_context: &mut FnContext) -> EvalRes<Value> {
    let l_val = eval_expr(l, fn_context)?;
    let r_val = eval_expr(r, fn_context)?;

    match (l_val, r_val) {
        (Num(l_val), Num(r_val)) => eval_i32_expr(l_val, op, r_val),
        (Bool(l_val), Bool(r_val)) => eval_bool_expr(l_val, op, r_val),
        _ => Err(EvalErr::TypeMismatch(String::from(
            "Can not evaluate an operation between a bool and an i32.",
        ))),
    }
}

// Evaluates a complete binomial tree to a single integer or bool.
pub fn eval_expr(e: Expr, fn_context: &mut FnContext) -> EvalRes<Value> {
    //let context = fn_context.get_last_context()?;
     match e {
        Expr::Num(num) => Ok(Num(num)),
        Expr::Bool(b) => Ok(Bool(b)),
        Expr::Var(s) => fn_context.get_last_context()?.get_val(&s),
        Expr::BinOp(left, op, right) => eval_bin_expr(*left, op, *right, fn_context),
        Expr::VarOp(var, op, expr) => {
            let key = String::from(*var);
            let expr_val = eval_expr(*expr, fn_context)?;

            match op {
                Op::VarOp(VarToken::Assign) => fn_context.get_last_context()?.update_var(&key, &expr_val),
                _ => eval_var_op(&key, op, &expr_val, fn_context.get_last_context()?),
            }
        },
        Expr::Let(var, _, expr) => assign_var(*var, *expr, fn_context), // ignore type for now
        Expr::If(expr, block) => eval_if(*expr, block, fn_context),
        //Expr::FuncCall(fn_call) => eval_fn_call(fn_call, fn_context),
        _ => Err(EvalErr::NotImplemented),
    }
}

// Assigns value to variable. Store it in current scope.
fn assign_var(var: Expr, expr: Expr, fn_context: &mut FnContext) -> EvalRes<Value> {   
    let id = String::from(var);
    let expr_val = eval_expr(expr, fn_context)?;
    fn_context.get_last_context()?.insert_to_current_scope(&id, &expr_val);
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
        },
        Op::VarOp(VarToken::MinEq) => {
            let new_val = Num(old_val - expr_val);
            context.update_var(&key, &new_val)
        },
        Op::VarOp(VarToken::MulEq) => {
            let new_val = Num(old_val * expr_val);
            context.update_var(&key, &new_val)
        },
        _ => Err(EvalErr::WrongOp("Not a variable operator.".to_string()))
    }
}

fn eval_if(e: Expr, block: Block, fn_context: &mut FnContext) -> EvalRes<Value> {
    let condition = eval_expr(e, fn_context)?;
    let res: EvalRes<Value>;

    match condition {
        Bool(true) => {
            res = eval_block(block, fn_context);
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

// Evaluates a complete block. Returns the value from the last instruction evaluated.
pub fn eval_block(block: Block, fn_context: &mut FnContext) -> EvalRes<Value> {
    let context = fn_context.get_last_context()?;
    context.new_scope();
    let mut res: EvalRes<Value> =
        Err(EvalErr::NotFound("No expressions found.".to_string()));

    for e in block {
        res = eval_expr(e, fn_context);
    }
    // Should drop the scope after here
    //context.drop_current_scope();
    res
}

// TODO
// Evaluates a function call in the program.
// Args should be mapped to the same name as the parameters, then a scope with
// said args should first be created. After that a block should be evaluated
// as normal. 
pub fn eval_fn_call(fn_call: FunctionCall, fn_context: &mut FnContext) { // -> EvalRes<Value> {

}

// pub fn eval_while() {}
// pub fn eval_return() {}

// Main entry
// Should evaluate the program starting from "main"
pub fn eval_program(fn_list: Functions) {
    let fn_context = FnContext::new();
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
}