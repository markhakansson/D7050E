extern crate d7050e;

use crate::d7050e::parser::*;
//use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;

fn main() {
    //let sum = "   10  + 2 +  3  +  4  + --------     3000         ";
    //let decl = "let a: i32 = 3 + 2 - 5;";
    //let tree_sum = parse_expr(sum);
    //let tree_decl = parse_expr(decl);
    //println!("{:#?}", tree_sum);
    //println!("{:#?}", tree_decl);

    //let sum = "1+(1+((3+50)-2))+(2)";

    //let sum = "a || (b + 1) && ((a-5) == true)";
    let sum = "9 + 3 / 8";
    let (_, tree_sum) = parse_expr(sum).unwrap();
    println!("{:#?}", tree_sum);

    let decl = "let a: i32 = 3 + 2 - 5;";
    let (_, tree_decl) = parse_expr(decl).unwrap();
    //walk_tree(tree_decl);
    println!("{:#?}", tree_decl);
}
