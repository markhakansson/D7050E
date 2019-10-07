extern crate nom;

use crate::ast::*;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, multispace0},
    combinator::map,
    multi::{fold_many0, many0},
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

// Parses the name of any variable to the AST Var type.
fn parse_var(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        map(alphanumeric1, |s: &str| Expr::Var(s.to_string())),
        multispace0,
    )(input)
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    delimited(
        multispace0,
        alt((
            map(tag("i32"), |_| Type::Int32),
            map(tag("bool"), |_| Type::Bool),
            map(tag("()"), |_| Type::Void),
        )),
        multispace0,
    )(input)
}

// Parses declaration of a variable
fn parse_declaration(input: &str) -> IResult<&str, Expr> {
    let (substring, (id, type_lit, expr)): (&str, (Expr, Type, Expr)) = tuple((
        preceded(
            multispace0,
            preceded(tag("let"), preceded(multispace0, parse_var)),
        ),
        preceded(
            multispace0,
            preceded(tag(":"), preceded(multispace0, parse_type)),
        ),
        preceded(
            multispace0,
            preceded(
                tag("="),
                delimited(multispace0, parse_right_expr, multispace0),
            ),
        ),
    ))(input)?;

    Ok((substring, Expr::Let(Box::new(id), type_lit, Box::new(expr))))
}

// Parses any i32. Handles multiple negative signs.
fn parse_i32(input: &str) -> IResult<&str, Expr> {
    let (substring, sign) = fold_many0(
        delimited(multispace0, tag("-"), multispace0),
        1,
        |mut sign: i32, _| {
            sign *= -1;
            sign
        },
    )(input)?;

    let (substring, digit) = delimited(multispace0, digit1, multispace0)(substring)?;

    Ok((substring, Expr::Num(digit.parse::<i32>().unwrap() * sign)))
}

// Helper function to parse parentheses
fn parse_parens_expr(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        delimited(tag("("), parse_right_expr, tag(")")),
        multispace0,
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            map(tag("true"), |_| Expr::Bool(true)),
            map(tag("false"), |_| Expr::Bool(false)),
        )),
        multispace0,
    )(input)
}

fn parse_bool_op(input: &str) -> IResult<&str, Op> {
    delimited(
        multispace0,
        alt((
            map(tag("&&"), |_| Op::BoolOp(BoolToken::And)),
            map(tag("||"), |_| Op::BoolOp(BoolToken::Or)),
            map(tag("!"), |_| Op::BoolOp(BoolToken::Not)),
        )),
        multispace0,
    )(input)
}

fn parse_rel_op(input: &str) -> IResult<&str, Op> {
    delimited(
        multispace0,
        alt((
            map(tag("=="), |_| Op::RelOp(RelToken::Equal)),
            map(tag("<"), |_| Op::RelOp(RelToken::Leq)),
            map(tag(">"), |_| Op::RelOp(RelToken::Geq)),
            map(tag("!="), |_| Op::RelOp(RelToken::Neq)),
        )),
        multispace0,
    )(input)
}

fn parse_math_op(input: &str) -> IResult<&str, Op> {
    delimited(
        multispace0,
        alt((
            map(tag("/"), |_| Op::MathOp(MathToken::Division)),
            map(tag("%"), |_| Op::MathOp(MathToken::Modulo)),
            map(tag("*"), |_| Op::MathOp(MathToken::Multiply)),
            map(tag("-"), |_| Op::MathOp(MathToken::Minus)),
            map(tag("+"), |_| Op::MathOp(MathToken::Plus)),
        )),
        multispace0,
    )(input)
}

fn parse_var_op(input: &str) -> IResult<&str, Op> {
    delimited(
        multispace0,
        alt((
            map(tag("="), |_| Op::VarOp(VarToken::Assign)),
            map(tag("+="), |_| Op::VarOp(VarToken::PlusEq)),
            map(tag("-="), |_| Op::VarOp(VarToken::MinEq)),
            map(tag("*="), |_| Op::VarOp(VarToken::MulEq)),
        )),
        multispace0,
    )(input)
}

fn parse_any_op(input: &str) -> IResult<&str, Op> {
    alt((parse_bool_op, parse_math_op, parse_rel_op))(input)
}

// Parses arithmetic and logical binomial expressions.
fn parse_bin_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((
                alt((parse_bool, parse_i32, parse_parens_expr, parse_var)),
                parse_any_op,
                parse_bin_expr,
            )),
            |(left, op, right)| Expr::BinOp(Box::new(left), op, Box::new(right)),
        ),
        parse_bool,
        parse_i32,
        parse_parens_expr,
        parse_var,
    ))(input)
}

fn parse_single_param(input: &str) -> IResult<&str, Param> {
    let (substring, (id, id_type)) = tuple((terminated(parse_var, tag(":")), parse_type))(input)?;

    let param = Param::new(id.into(), id_type);

    Ok((substring, param))
}

fn parse_fn_params(input: &str) -> IResult<&str, Vec<Param>> {
    delimited(
        multispace0,
        delimited(
            tag("("),
            many0(alt((
                parse_single_param,
                preceded(tag(","), parse_single_param),
            ))),
            tag(")"),
        ),
        multispace0,
    )(input)
}

// Parses blocks of keyword statements.
pub fn parse_block(input: &str) -> IResult<&str, Vec<Expr>> {
    delimited(
        tag("{"),
        many0(alt((
            terminated(parse_keyword, terminated(tag(";"), multispace0)),
            parse_return,
        ))),
        tag("}"),
    )(input)
}

// Parses return-statements
fn parse_return(input: &str) -> IResult<&str, Expr> {
    let (substring, ret) = preceded(tag("return"), parse_right_expr)(input)?;

    Ok((substring, Expr::Return(Box::new(ret))))
}

fn parse_function(input: &str) -> IResult<&str, Function> {
    let (substring, (id, params, return_type, block)) = tuple((
        delimited(multispace0, preceded(tag("fn"), parse_var), multispace0),
        parse_fn_params,
        delimited(multispace0, preceded(tag("->"), parse_type), multispace0),
        parse_block,
    ))(input)?;

    let func = Function::new(id.into(), params, Block::new(block), return_type);
    Ok((substring, func))
}

// Parses lonely if statements
fn parse_if(input: &str) -> IResult<&str, Expr> {
    let (substring, (_, exp, block)) = tuple((
        delimited(multispace0, tag("if"), multispace0),
        alt((parse_bin_expr, parse_var_expr)),
        delimited(multispace0, parse_block, multispace0),
    ))(input)?;

    Ok((substring, Expr::If(Box::new(exp), Block::new(block) )))
}

//fn parse_else(input: &str) -> IResult<&str,Expr> {}

fn parse_while(input: &str) -> IResult<&str, Expr> {
    let (substring, (_, expr, block)) = tuple((
        delimited(multispace0, tag("while"), multispace0),
        parse_right_expr,
        parse_block,
    ))(input)?;

    Ok((substring, Expr::While(Box::new(expr), Block::new(block) )))
}

// Parses variable assignments where the variable has already
// been declared. E.g. 'a = 3;'.
fn parse_var_expr(input: &str) -> IResult<&str, Expr> {
    let (substring, (var, op, expr)) = tuple((parse_var, parse_var_op, parse_right_expr))(input)?;

    Ok((substring, Expr::VarOp(Box::new(var), op, Box::new(expr))))
}

// Parses keywords such as 'let', 'fn', 'if' etc.
fn parse_keyword(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            parse_return,
            parse_declaration,
            parse_if,
            parse_while,
            parse_var_expr,
            parse_func_call,
        )),
        multispace0,
    )(input)
}

// Parses right-hand expressions
fn parse_right_expr(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((parse_func_call, parse_bin_expr)),
        multispace0,
    )(input)
}

fn parse_func_call(input: &str) -> IResult<&str, Expr> {
    let (substring, (fn_name, args)) = tuple((parse_var, parse_fn_args))(input)?;

    Ok((
        substring,
        Expr::FuncCall(FunctionCall::new(fn_name.into(), Args::new(args) )),
    ))
}

fn parse_single_arg(input: &str) -> IResult<&str, Expr> {
    let (substring, val) = terminated(parse_right_expr, multispace0)(input)?;

    Ok((substring, val))
}

fn parse_fn_args(input: &str) -> IResult<&str, Vec<Expr>> {
    delimited(
        multispace0,
        delimited(
            tag("("),
            many0(alt((
                parse_single_arg,
                preceded(tag(","), parse_single_arg),
            ))),
            tag(")"),
        ),
        multispace0,
    )(input)
}

// TODO: this should be the only public function in the parser.
// Main entry to parse a complete program
/* pub fn parse_program(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(delimited(multispace0, parse_keyword, multispace0))(input)
} */
pub fn parse_program(input: &str) -> IResult<&str, Functions> {
    many0(delimited(multispace0, parse_function, multispace0))(input)
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn test_parse_i32() {
        assert_eq!(parse_i32("3"), Ok(("", Expr::Num(3 as i32))));
        assert_eq!(parse_i32("-3"), Ok(("", Expr::Num(-3 as i32))));
        assert_eq!(parse_i32("500 + 50"), Ok(("+ 50", Expr::Num(500 as i32))));
        assert_eq!(
            parse_i32("- - 1000 --100 "),
            Ok(("--100 ", Expr::Num(1000 as i32)))
        );
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(parse_bool("true"), Ok(("", Expr::Bool(true))));
        assert_eq!(
            parse_bool("  false && true"),
            Ok(("&& true", Expr::Bool(false)))
        );
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type("i32"), Ok(("", Type::Int32)));
        assert_eq!(parse_type(" bool : true;"), Ok((": true;", Type::Bool)))
    }

    #[test]
    fn test_parse_right_expr() {
        assert_eq!(parse_right_expr("a = 2").is_ok(), true);
        assert_eq!(parse_right_expr("1 + 3 + 4 / 50").is_ok(), true);
    }

    #[test]
    fn test_parse_keyword() {
        assert_eq!(parse_keyword("return 1 + 2").is_ok(), true);
        assert_eq!(parse_keyword("let a: i32 = 1 + 3;").is_ok(), true);
        assert_eq!(parse_keyword("if a == true { return 0; }").is_ok(), true);
        assert_eq!(parse_keyword("while true { return 1; }").is_ok(), true);
        assert_eq!(parse_keyword("a += 5;").is_ok(), true);
        assert_eq!(
            parse_keyword(
                "
            fn func(a: i32, b: bool, c :i32) -> i32 {
                let d: bool = a == c;
                let hej: bool = ((1+3) == 4) == true;
                if b && d == true {
                    return 0;
                };
            }"
            )
            .is_ok(),
            true
        );
    }

    #[test]
    fn test_parse_program() {
        let program = "
        fn test(b: bool) -> i32 {
            let: a: i32 = 0;
            if b {
                a = 50;
            };
            return a;
        }
        
        fn main() -> () {
            let b: bool = true;
            let i: i32 = 5;
            let a: i32 = 0;
            while (b) {
                a += test(b);
                b = false;
            };
        }";
        assert_eq!(parse_program(program).is_ok(), true);
    }
}
