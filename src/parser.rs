extern crate nom;

use crate::ast::*;

use nom::{
    branch::{alt, permutation},
    bytes::complete::{is_not, tag, take_till1, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, anychar, digit1, multispace0},
    combinator::{map, map_parser, peek},
    multi::{fold_many0, many0_count},
    named,
    sequence::{delimited, pair, preceded, terminated, tuple},
    FindSubstring, IResult,
};

pub fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    preceded(
        multispace0,
        //map(alphanumeric1, |s: &str| Identifier::new(s)),
        map(alphanumeric1, |s: &str| Expr::Var(s.to_string())),
    )(input)
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("i32"), |_| Type::Int32),
        map(tag("bool"), |_| Type::Bool),
    ))(input)
}

// Parses declaration of a variable
pub fn parse_declaration(input: &str) -> IResult<&str, Expr> {
    let (_, (id, type_lit, expr)): (&str, (Expr, Type, Expr)) = tuple((
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
            preceded(tag("="), preceded(multispace0, parse_expr)),
        ),
    ))(input)?;

    Ok(("", Expr::Let(Box::new(id), type_lit, Box::new(expr))))
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
pub fn parse_parens_expr(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        delimited(tag("("), parse_expr, tag(")")),
        multispace0,
    )(input)
}

pub fn parse_bool(input: &str) -> IResult<&str, Expr> {
    delimited(
        multispace0,
        alt((
            map(tag("true"), |_| Expr::Bool(true)),
            map(tag("false"), |_| Expr::Bool(false)),
            map(alpha1, |i: &str| Expr::Var(i.to_string())),
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
                alt((parse_bool, parse_i32, parse_parens_expr)),
                parse_any_op,
                parse_bin_expr,
            )),
            |(left, op, right)| Expr::BinOp(Box::new(left), op, Box::new(right)),
        ),
        parse_bool,
        parse_i32,
        parse_parens_expr,
    ))(input)
}

pub fn parse_args(input: &str) {}

pub fn parse_function(input: &str) {}

// Parse any type of expression
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_declaration, parse_bin_expr))(input)
}
