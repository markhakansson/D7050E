use crate::ast::*;
use crate::context::*;
use crate::parser::*;
use std::fmt;

pub type TypeRes<T> = Result<T, TypeErr>;

/* #[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeErr {
    Mismatch(String),
    Unknown(String),
} */

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeErr(pub String);

/// Type checks any expression.
fn tc_expr(expr: Expr, fn_list: &mut Functions, fn_context: &mut ContextStack<Type>) -> TypeRes<Type> {
    match expr.clone() {
        Expr::Num(_) => Ok(Type::Int32),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Var(s) => fn_context.get_last_context()?.get_val(&s),
        Expr::BinOp(l, op, r) => tc_bin_expr(*l, op, *r, fn_list, fn_context),
        Expr::VarOp(var, op, val) => tc_var_op(*var, op, *val, fn_list, fn_context),
        Expr::Let(var, var_type, expr) => tc_let_expr(*var, var_type, *expr, fn_list, fn_context),
        Expr::If(_, _) | Expr::While(_, _) => tc_cond_branch(expr, fn_list, fn_context),
        Expr::Return(expr) => tc_return(*expr, fn_list, fn_context),
        Expr::FuncCall(fn_call) => tc_fn_call(fn_call, fn_list, fn_context),
        _ => Err(TypeErr("not yet implemented".to_string())),
    }
}

/// Returns a formatted TypeErr for when there is no implementation for operation between types.
fn err_no_impl(first: Type, op: Op, second: Type) -> TypeRes<Type> {
    Err(TypeErr(format!(
        "no implementation for ´{{{:?}}} {} {{{:?}}}´",
        first, op, second
    )))
}

/// Type check expressions between two i32 values. Checks if the operation is valid,
/// returns the type of the operation if successful, otherwise returns an error with
/// a message.
fn tc_i32_expr(first: Type, op: Op, second: Type) -> TypeRes<Type> {
    if (first, second) != (Type::Int32, Type::Int32) {
        err_no_impl(first, op, second)
    } else {
        match op {
            Op::MathOp(MathToken::Division) => Ok(Type::Int32),
            Op::MathOp(MathToken::Multiply) => Ok(Type::Int32),
            Op::MathOp(MathToken::Plus) => Ok(Type::Int32),
            Op::MathOp(MathToken::Minus) => Ok(Type::Int32),
            Op::MathOp(MathToken::Modulo) => Ok(Type::Int32),
            Op::RelOp(RelToken::Equal) => Ok(Type::Int32),
            Op::RelOp(RelToken::Geq) => Ok(Type::Int32),
            Op::RelOp(RelToken::Leq) => Ok(Type::Int32),
            Op::RelOp(RelToken::Neq) => Ok(Type::Int32),
            _ => err_no_impl(first, op, second),
        }
    }
}

/// Type check boolean expressions on two bool values. Checks if the operation is valid,
/// returns the type of the operation if successful, otherwise returns an error with
/// a message.
fn tc_bool_expr(first: Type, op: Op, second: Type) -> TypeRes<Type> {
    if (first, second) != (Type::Bool, Type::Bool) {
        err_no_impl(first, op, second)
    } else {
        match op {
            Op::BoolOp(BoolToken::And) => Ok(Type::Bool),
            Op::BoolOp(BoolToken::Or) => Ok(Type::Bool),
            Op::RelOp(RelToken::Equal) => Ok(Type::Bool),
            Op::RelOp(RelToken::Geq) => Ok(Type::Bool),
            Op::RelOp(RelToken::Leq) => Ok(Type::Bool),
            Op::RelOp(RelToken::Neq) => Ok(Type::Bool),
            _ => err_no_impl(first, op, second),
        }
    }
}

/// Type check expressions between two values. Checks if the operation is valid,
/// returns the type of the operation if successful, otherwise returns an error with
/// a message.
fn tc_bin_expr(
    l: Expr,
    op: Op,
    r: Expr,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let l_type = tc_expr(l.clone(), fn_list, fn_context)?;
    let r_type = tc_expr(r.clone(), fn_list, fn_context)?;

    let res = match (l_type, r_type) {
        (Type::Bool, Type::Bool) => tc_bool_expr(l_type, op, r_type),
        (Type::Int32, Type::Int32) => tc_i32_expr(l_type, op, r_type),
        _ => err_no_impl(l_type, op, r_type),
    };

    match res {
        Ok(op_type) => Ok(op_type),
        Err(e) => Err(TypeErr(format!(
            "on values ´{}´ and ´{}´: {} ",
            String::from(l),
            String::from(r),
            e.0
        ))),
    }
}
/// Type checks variable operations such as ´<var> = <value>´.
fn tc_var_op(
    var: Expr,
    op: Op,
    val: Expr,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let var_type = tc_expr(var.clone(), fn_list, fn_context)?;
    let val_type = tc_expr(val.clone(), fn_list, fn_context)?;

    if var_type == val_type {
        Ok(var_type)
    } else {
        Err(TypeErr(format!(
            "when changing the variable to ´{} {} {}´ -> expected type {{{}}}, found type {{{}}}",
            String::from(var),
            op,
            String::from(val),
            String::from(var_type),
            String::from(val_type)
        )))
    }
}

/// Type checks variable declarations i.e. ´let´ statements.
fn tc_let_expr(
    var: Expr,
    var_type: Type,
    expr: Expr,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let expr_type: Type; // = tc_expr(expr.clone(), fn_list, fn_context)?;

    match tc_expr(expr.clone(), fn_list, fn_context) {
        Ok(expr_type_return) => expr_type = expr_type_return,
        Err(e) => {
            return Err(TypeErr(format!(
                "when assigning variable ´{}´ -> {}",
                String::from(var),
                e.0
            )))
        }
    }

    if var_type == expr_type {
        fn_context
            .get_last_context()?
            .insert_to_current_scope(&String::from(var), &var_type);
        Ok(var_type)
    } else {
        Err(TypeErr(format!(
            "when assigning variable ´{}´ -> expected type {{{}}}, found type {{{}}}",
            String::from(var),
            String::from(var_type),
            String::from(expr_type)
        )))
    }
}

/// Type checks conditional branches e.g. ´if´ and ´while´ statements.
fn tc_cond_branch(
    expr: Expr,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let cond_type: Type;
    let block: Block;

    match expr.clone() {
        Expr::If(cond, cond_block) | Expr::While(cond, cond_block) => {
            cond_type = tc_expr(*cond, fn_list, fn_context)?;
            block = cond_block;
        }
        _ => {
            return Err(TypeErr(format!(
                "{} is not implemented!",
                String::from(expr)
            )))
        }
    }

    if cond_type == Type::Bool {
        tc_block(block, fn_list, fn_context)
    } else {
        match expr {
            Expr::If(_, _) => Err(TypeErr(format!(
                "in ´if´ statement -> expected {{Bool}} found type {{{}}}",
                String::from(cond_type)
            ))),
            Expr::While(_, _) => Err(TypeErr(format!(
                "in ´while´ statement -> expected {{Bool}} found type {{{}}}",
                String::from(cond_type)
            ))),
            _ => unreachable!(),
        }
    }
}

fn tc_block(
    block: Block,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    fn_context.get_last_context()?.new_scope();

    let mut res: TypeRes<Type> = Err(TypeErr("Error: No expressions found!".to_string()));
    for expr in block.content.iter() {
        res = tc_expr(expr.clone(), fn_list, fn_context);
        match res {
            Ok(_) => continue,
            Err(e) => {
                return Err(e);
            }
        }
    }

    fn_context.get_last_context()?.drop_current_scope();

    res
}

fn tc_function(
    func: &Function,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let context = fn_context.new_context()?;
    context.new_scope();

    // Insert function parameters and types into the scope
    for param in func.params.iter() {
        context.insert_to_current_scope(&param.name, &param.param_type);
    }

    // Insert return type on the top for easy referencing.
    // Needs a better way to store return type, as a program might use this variable name internally
    context.insert_to_current_scope(&"return_type".to_string(), &func.return_type);

    let res = tc_block(func.block.clone(), fn_list, fn_context);

    fn_context.drop_current_context();

    match res {
        Ok(res_type) => Ok(res_type),
        Err(e) => Err(TypeErr(format!("in function ´{}´-> {}", func.name, e.0))),
    }
}

fn tc_return(
    expr: Expr,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let ret_type = fn_context.get_last_context()?.get_val(&"return_type")?;
    let expr_type = tc_expr(expr, fn_list, fn_context)?;

    if expr_type == ret_type {
        Ok(expr_type)
    } else {
        Err(TypeErr(format!(
            "expected return type {{{}}}, found type {{{}}}",
            String::from(ret_type),
            String::from(expr_type)
        )))
    }
}

fn tc_fn_call(
    fn_call: FunctionCall,
    fn_list: &mut Functions,
    fn_context: &mut ContextStack<Type>,
) -> TypeRes<Type> {
    let fn_res: TypeRes<Function> = fn_list.get_fn(fn_call.name);
    let fn_content = fn_res?;

    let params = fn_content.params;
    let fn_type = fn_content.return_type;
    let args = fn_call.args;
    let mut res: TypeRes<Type> = Err(TypeErr("Error: no parameters in function".to_string()));

    // Check wheter params and args are same type
    for i in 0..params.len() {
        let arg_type = tc_expr(args.content[i].clone(), fn_list, fn_context)?;
        let param_type = params[i].param_type;
        if arg_type != param_type {
            return Err(TypeErr(format!(
                "in function call to ´{}´-> expected argument type {{{}}}, found type {{{}}}",
                String::from(fn_content.name),
                String::from(param_type),
                String::from(arg_type),
            )));
        } else {
            res = Ok(param_type);
        }
    }

    // If ok - returns the function's return type.
    match res {
        Ok(_) => Ok(fn_type),
        Err(e) => Err(e),
    }
}

/// Type checks a complete program.
pub fn tc_program(fn_list: &mut Functions) -> TypeRes<Type> {
    let mut fn_context: ContextStack<Type> = ContextStack::new();
    let mut res: TypeRes<Type> = Err(TypeErr("Error: No functions found.".to_string()));

    for func in fn_list.clone() {
        res = tc_function(&func, fn_list, &mut fn_context);

        match res {
            Ok(_) => continue,
            Err(e) => return Err(e),
        }
    }

    res
}

pub fn test() {
    let mut fn_context: ContextStack<Type> = ContextStack::new();
    fn_context.new_context().unwrap().new_scope();

    /*     let expr = "{
        let a: bool = true;
        let b: i32 = 1;
        while a {
            b+=a;
       };
    }";

    let tree = parse_block(expr).unwrap().1;

    let tc = tc_block(Block::new(tree), &mut fn_context);
    println!("{:#?}", tc); */

    let expr = "
    fn func1(a: i32) -> i32 {
        let hello: bool = true;
        let fifty: i32 = k;
        let a: i32 = func2(hello);
        return a;
    }

    fn func2(b: i32) -> i32 {
        return b*5;   
    }";
    let mut tree = parse_program(expr).unwrap().1;
    let tc = tc_program(&mut tree);

    println!("{:#?}", tc);
}
