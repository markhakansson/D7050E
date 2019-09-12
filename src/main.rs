extern crate nom;

#[macro_use]
extern crate derive_more;

use std::error::Error;
use std::fmt;
use std::result::Result;

use nom::{  IResult,
            branch::{alt,permutation},
            FindSubstring,
            named,
            character::{complete::{ digit1,
                                    alpha1,
                                    multispace0,
                                    anychar,
                                    alphanumeric1
                                    },
                        },
            sequence::{delimited,preceded,pair,tuple,terminated},
            bytes::complete::{tag,is_not,take_till1,take_while1,take_while},
            combinator::{map,map_parser,peek},
            multi::{many0_count, fold_many0}
        };

// Identifier for variables etc.
#[derive(PartialEq,Debug,Eq)]
struct Identifier(String);

impl Identifier {
    fn new(name: &str) -> Identifier {
        Identifier(name.to_string())
    }
}

#[derive(Debug,PartialEq,Eq)]
enum Operator {
    Minus,
    Plus,
    Multiply,
    Division,
    Modulo,
}

#[derive(Debug,PartialEq,Eq)]
enum Type {
    Int32,
    Bool,
}

#[derive(Debug)]
enum Expr {
    BinOp(Box<Expr>, Operator, Box<Expr>),
    Num(i32),
    Bool(bool),
    Let(Identifier, Type, Box<Expr>),
}

fn parse_identifier(input: &str) -> IResult<&str,Identifier> {
    preceded(
        multispace0,
        map(alphanumeric1, |s: &str| Identifier::new(s))
    )(input)
}

// Parses type
fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("i32"), |_| Type::Int32),
        map(tag("bool"), |_| Type::Bool)
    ))(input)
}

// Parses declaration of a variable
fn parse_declaration(input: &str) -> IResult<&str,Expr> {
    let (_,(id, type_lit, expr)): (&str,(Identifier,Type,Expr)) = tuple((
        preceded(
            multispace0,
            preceded(
                tag("let"),
                preceded(
                    multispace0,
                    parse_identifier
                )
            )
        ),
        preceded(
            multispace0,
            preceded(
                tag(":"),
                preceded(
                    multispace0, 
                    parse_type
                )
            )
        ),
        preceded(
            multispace0,
            preceded(
                tag("="),
                preceded(
                    multispace0,
                    parse_expr
                )
            )
        )
    ))(input)?;

    Ok(("",Expr::Let(id,type_lit,Box::new(expr))))

}

fn parse_op(input: &str) -> IResult<&str,Operator> {
    preceded(
        multispace0,
        terminated(
            alt((
                map(tag("/"), |_| Operator::Division),
                map(tag("%"), |_| Operator::Modulo),
                map(tag("*"), |_| Operator::Multiply),
                map(tag("-"), |_| Operator::Minus),
                map(tag("+"), |_| Operator::Plus)
            )),
            multispace0
        )
    )(input)    
}

fn parse_i32(input: &str) -> IResult<&str,i32> {
    let (substring,sign) = fold_many0(
        preceded(
            multispace0,
            terminated(
                tag("-"),
                multispace0
            )
        ),
        1,
        |mut sign: i32, _| {
            sign = sign*-1;
                sign
        }
    )(input)?;

    let (substring,digit) = preceded(
        multispace0,
        terminated(
            digit1,
            multispace0
        )
    )(substring)?;

    Ok((substring,digit.parse::<i32>().unwrap() * sign))

}

// NEEDS a reimplementation
// Should fix priority between operators
fn parse_expr(input: &str) -> IResult<&str, Expr> {

    // Find i32
    if let Ok((substring, digit)) = parse_i32(input) {
        if substring == ";" {
            return Ok(("",Expr::Num(digit)))
        } else {
    
            let (substring,operator) = parse_op(substring).unwrap();

            return Ok ((
                "", 
                Expr::BinOp(
                Box::new(Expr::Num(digit)),
                operator,
                Box::new(parse_expr(substring).unwrap().1)
                )
            ))
        }
    }

    panic!()

} 

fn main() {
    let sum = "   10  + 2 +  3  +  4  + --------     3000         ;";
    let tree = parse_expr(sum);
    println!("{:#?}",tree); 
}
