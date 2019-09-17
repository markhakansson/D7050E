extern crate d7050e;

use crate::d7050e::parser::*;
use crate::d7050e::ast::*;

fn main() {
    //let sum = "   10  + 2 +  3  +  4  + --------     3000         ";
    //let decl = "let a: i32 = 3 + 2 - 5;";
    //let tree_sum = parse_expr(sum);
    //let tree_decl = parse_expr(decl);
    //println!("{:#?}", tree_sum);
    //println!("{:#?}", tree_decl);

    //let nested = "(1+3)";
    //let tree = parse_nested(nested);
    //println!("{:#?}",tree);

    let booltest = "let a: bool = true;";
    let tree = parse_declaration(booltest);
    println!("{:#?}", tree);
}
