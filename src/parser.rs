extern crate nom;

use crate::ast::*;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take_till1, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, anychar, digit1, multispace0},
    combinator::{map, map_parser, peek},
    multi::{fold_many0, many0},
    named,
    sequence::{delimited, pair, preceded, terminated, tuple},
    FindSubstring, IResult,
};

pub fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        map(alphanumeric1, |s: &str| Expr::Var(s.to_string())),
        multispace0,
    )(input)
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    delimited(
        multispace0,
        alt((
            map(tag("i32"), |_| Type::Int32),
            map(tag("bool"), |_| Type::Bool),
        )),
        multispace0,
    )(input)
}

// Parses declaration of a variable
pub fn parse_declaration(input: &str) -> IResult<&str, Expr> {
    let (substring, (id, type_lit, expr)): (&str, (Expr, Type, Expr)) = tuple((
        preceded(
            multispace0,
            preceded(tag("let"), preceded(multispace0, parse_identifier)),
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
pub fn parse_i32(input: &str) -> IResult<&str, Expr> {
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

pub fn parse_bool(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            map(tag("true"), |_| Expr::Bool(true)),
            map(tag("false"), |_| Expr::Bool(false)),
        )),
        multispace0,
    )(input)
}

pub fn parse_bool_op(input: &str) -> IResult<&str, Op> {
    delimited(
        multispace0,
        alt((
            map(tag("&&"), |_| Op::BoolOp(BoolToken::And)),
            map(tag("||"), |_| Op::BoolOp(BoolToken::Or)),
            map(tag("!"), |_| Op::BoolOp(BoolToken::Not)),
            map(tag("<"), |_| Op::BoolOp(BoolToken::Leq)),
            map(tag(">"), |_| Op::BoolOp(BoolToken::Geq)),
            map(tag("=="), |_| Op::BoolOp(BoolToken::Equal)),
            map(tag("!="), |_| Op::BoolOp(BoolToken::Neq)),
        )),
        multispace0,
    )(input)
}

pub fn parse_math_op(input: &str) -> IResult<&str, Op> {
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

pub fn parse_any_op(input: &str) -> IResult<&str, Op> {
    alt((parse_bool_op, parse_math_op))(input)
}

// Parses arithmetic and logical binomial expressions.
pub fn parse_bin_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(
            tuple((
                alt((parse_bool, parse_i32, parse_parens_expr, parse_identifier)),
                parse_any_op,
                parse_bin_expr,
            )),
            |(left, op, right)| Expr::BinOp(Box::new(left), op, Box::new(right)),
        ),
        parse_bool,
        parse_i32,
        parse_parens_expr,
        parse_identifier,
    ))(input)
}

fn parse_single_arg(input: &str) -> IResult<&str, Param> {
    let (substring, (id, id_type)) =
        tuple((terminated(parse_identifier, tag(":")), parse_type))(input)?;

    let param = Param::new(id.into(), id_type);

    Ok((substring, param))
}

pub fn parse_fn_args(input: &str) -> IResult<&str, Vec<Param>> {
    many0(alt((
        parse_single_arg,
        preceded(tag(","), parse_single_arg),
    )))(input)
}

// Parses blocks of keyword statements.
pub fn parse_block(input: &str) -> IResult<&str, Vec<Expr>> {
    many0(alt((
        terminated(parse_keyword, terminated(tag(";"), multispace0)),
        parse_return,
    )))(input)
}

// Parses return-statements
pub fn parse_return(input: &str) -> IResult<&str, Expr> {
    let (substring, ret) = preceded(tag("return"), parse_right_expr)(input)?;

    Ok((substring, Expr::Return(Box::new(ret))))
}

pub fn parse_function(input: &str) -> IResult<&str, Expr> {
    let (substring, (id, params, return_type, block)) = tuple((
        delimited(
            multispace0,
            preceded(tag("fn"), parse_identifier),
            multispace0,
        ),
        delimited(tag("("), parse_fn_args, tag(")")),
        delimited(multispace0, preceded(tag("->"), parse_type), multispace0),
        delimited(tag("{"), parse_block, tag("}")),
    ))(input)?;

    let func = Function::new(id.into(), params, block, return_type);
    Ok((substring, Expr::Func(func)))
}

// wip
pub fn parse_if(input: &str) {
    let (subsstring, exp) =
        preceded(tag("if"), preceded(multispace0, parse_bin_expr))(input).unwrap();

    println!("{:#?}", exp)
}

//fn parse_else(input: &str) -> IResult<&str,Expr> {}

// Parses keywords such as 'let', 'fn', 'if' etc.
pub fn parse_keyword(input: &str) -> IResult<&str, Expr> {
    alt((parse_return, parse_declaration, parse_function))(input)
}

// Parses right-hand expressions
pub fn parse_right_expr(input: &str) -> IResult<&str, Expr> {
    parse_bin_expr(input)
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

}
