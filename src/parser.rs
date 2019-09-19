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

pub fn parse_identifier(input: &str) -> IResult<&str, Identifier> {
    preceded(
        multispace0,
        map(alphanumeric1, |s: &str| Identifier::new(s)),
    )(input)
}

// Parses type
pub fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("i32"), |_| Type::Int32),
        map(tag("bool"), |_| Type::Bool),
    ))(input)
}

// Parses declaration of a variable
pub fn parse_declaration(input: &str) -> IResult<&str, Expr> {
    let (_, (id, type_lit, expr)): (&str, (Identifier, Type, Expr)) = tuple((
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

    Ok(("", Expr::Let(id, type_lit, Box::new(expr))))
}

pub fn parse_bin_op(input: &str) -> IResult<&str, MathOp> {
    delimited(
        multispace0,
        alt((
            map(tag("/"), |_| MathOp::Division),
            map(tag("%"), |_| MathOp::Modulo),
            map(tag("*"), |_| MathOp::Multiply),
            map(tag("-"), |_| MathOp::Minus),
            map(tag("+"), |_| MathOp::Plus),
        )),
        multispace0
    )(input)
}

pub fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (substring, sign) = fold_many0(
        delimited(multispace0, tag("-"), multispace0),
        1,
        |mut sign: i32, _| {
            sign *= -1;
            sign
        },
    )(input)?;

    let (substring, digit) = delimited(multispace0, digit1, multispace0)(substring)?;

    Ok((substring, digit.parse::<i32>().unwrap() * sign))
}

pub fn parse_i32_expr(input: &str) -> IResult<&str, Expr> {
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
fn parse_parens(input: &str) -> IResult<&str,Expr> {
    delimited(
        multispace0,
        delimited(
            tag("("),
            parse_bin_expr,
            tag(")")
        ),
        multispace0
    )(input)
}

// it works??
// thanks Per!!
pub fn parse_bin_expr(input: &str) -> IResult<&str,Expr> {
    alt((
        map(
            tuple((
                alt((
                    parse_i32_expr,
                    parse_parens,
                )),
                parse_bin_op,
                parse_bin_expr
            )),
            |(left,op,right)| Expr::BinOp(Box::new(left),
                                    op,
                                    Box::new(right))
        ),
        parse_i32_expr,
        parse_parens
    ))(input)
}

// Old implementation
// Parses binomial/arithmetic expressions
pub fn parse_bin_expr_old(input: &str) -> IResult<&str, Expr> {
    let (substring, digit) = parse_i32(input)?;
    if substring == ";" || substring.is_empty() {
        return Ok(("", Expr::Num(digit)));
    } else {
        let (substring, operator) = parse_bin_op(substring)?;

        return Ok((
            "",
            Expr::BinOp(
                Box::new(Expr::Num(digit)),
                operator,
                Box::new(parse_expr(substring).unwrap().1),
            ),
        ));
    }
}



pub fn parse_bool(input: &str) -> IResult<&str, BoolState> {
    delimited(
        multispace0,
        alt((
                map(tag("true"), |_| BoolState::True),
                map(tag("false"), |_| BoolState::False),
        )),
        multispace0
    )(input)
}

pub fn parse_bool_op(input: &str) -> IResult<&str,BoolOp> {
    delimited(
        multispace0,
        alt((
            map(tag("&&"), |_| BoolOp::And),
            map(tag("||"), |_| BoolOp::Or),
            map(tag("!"), |_| BoolOp::Not),
            map(tag("<"), |_| BoolOp::Leq),
            map(tag(">"), |_| BoolOp::Geq),                
        )),
        multispace0
    )(input)
}

/* pub fn parse_logic_expr(input: &str) -> IResult<&str,Expr> {
    let (substring,)
} */

// Parse any type of expression
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_bin_expr, parse_declaration))(input)
}
