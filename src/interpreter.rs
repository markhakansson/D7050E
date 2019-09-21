use crate::ast::*;

#[derive(Debug, PartialEq, Eq)]
enum EvalValue {
    Num(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq, Eq)]
enum EvalError {}

// Evaluates <integer> <operator> <integer> to a single integer
fn eval_bin_expr(first: i32, operator: Op, second: i32) -> i32 {
    match operator {
        Op::MathOp(_) => eval_arithmetic_expr(first, operator, second),
        _ => panic!("Expected Op. "),
    }
}

fn eval_arithmetic_expr(first: i32, operator: Op, second: i32) -> i32 {
    match operator {
        Op::MathOp(MathToken::Division) => return first / second,
        Op::MathOp(MathToken::Multiply) => return first * second,
        Op::MathOp(MathToken::Plus) => return first + second,
        Op::MathOp(MathToken::Minus) => return first - second,
        Op::MathOp(MathToken::Modulo) => return first % second,
        _ => panic!(
            "Expected MathOp with MathToken. Found unknown value: {:?}",
            operator
        ),
    }
}

fn eval_bool_expr(first: bool, operator: Op, second: bool) -> bool {
    match operator {
        Op::BoolOp(BoolToken::And) => return first && second,
        Op::BoolOp(BoolToken::Equal) => return first == second,
        Op::BoolOp(BoolToken::Geq) => return first > second,
        Op::BoolOp(BoolToken::Leq) => return first < second,
        Op::BoolOp(BoolToken::Neq) => return first != second,
        Op::BoolOp(BoolToken::Or) => return first || second,
        _ => panic!(
            "Expected BoolOp with BoolToken. Found unknown value: {:?}",
            operator
        ),
    }
}

// Evaluates a complete binomial tree.
// Returns <expr> <operator> <expr> as a single integer.
pub fn eval_bin_tree(e: Expr) -> i32 {
    let mut sum: i32 = 0;

    match e {
        Expr::Var(var) => println!("var: {:?}", var),
        Expr::BinOp(left, op, right) => {
            sum = eval_arithmetic_expr(eval_bin_tree(*left), op, eval_bin_tree(*right))
        }

        Expr::Num(num) => sum = num,
        _ => (),
    }

    sum
}
