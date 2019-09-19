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

   //let sum = "1+(1+((3+50)-2))+(2)";
   let sum = " 1  +  ( 2   +   2 / (2 - 5 ) )    ";
   let tree = parse_bin_expr(sum);
   println!("{:#?}",tree);
}
