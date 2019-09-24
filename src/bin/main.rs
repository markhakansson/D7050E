extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

fn main() {
    let func = "fn test(a: i32, b: i32) -> i32 {
        let c: i32 = a + b + 100;
        return c;
    }";

    let fn_tree = parse_function(func);
    println!("{:#?}", fn_tree);

    /*     let block =
    "let a: i32 = 1 + 3;
    let b: bool = true;";

    let block_tree = parse_block(block);
    println!("{:#?}", block_tree); */

    /*     let decl = "let a: i32 = 1 + 3;}";
    let a = parse_declaration(decl);
    println!("{:#?}",a); */

    /* let ret = parse_return("return a\n");
    println!("{:#?}",ret); */

    /*     let ifs = "if a == b";
    let tree = parse_if(ifs);
    println!("{:#?}",tree); */
}
