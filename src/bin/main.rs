extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

fn main() {
    let func = "fn test(a: i32, b: i32) -> i32";
    let tree = parse_function(func);
    println!("{:#?}", tree);
}
