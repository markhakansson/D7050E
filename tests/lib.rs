extern crate simple_rustc;

use crate::simple_rustc::ast::*;
use crate::simple_rustc::parser::*;

/* #[test]
fn test_parse_bin_op() {
    assert_eq!(parse_bin_op("+"),Ok(("",MathOp::Plus)));
    assert_eq!(parse_bin_op("-  "),Ok(("",MathOp::Minus)));
    assert_eq!(parse_bin_op("  / 156  "),Ok(("156  ",MathOp::Division)));
    assert_eq!(parse_bin_op("       *"),Ok(("",MathOp::Multiply)));
    assert_eq!(parse_bin_op(" % 12"),Ok(("12",MathOp::Modulo)));
}
 */
/* #[test]
fn test_parse_bool() {
    assert_eq!(parse_bool("true"),Ok(("",BoolState::True)));
    assert_eq!(parse_bool(" false || false"), Ok(("|| false",BoolState::False)));
} */
/*
#[test]
fn test_parse_bool_op() {
    assert_eq!(parse_bool_op("&&"),Ok(("",BoolOp::And)));
    assert_eq!(parse_bool_op("|| false "),Ok(("false ",BoolOp::Or)));
    assert_eq!(parse_bool_op("!a || b"),Ok(("a || b",BoolOp::Not)));
    assert_eq!(parse_bool_op("  > x"),Ok(("x",BoolOp::Geq)));
    assert_eq!(parse_bool_op(" < <"),Ok(("<",BoolOp::Leq)));
} */
