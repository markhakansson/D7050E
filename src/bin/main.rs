extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

fn main() {
    let function = "
    fn func(a: i32, b: bool, c :i32) -> i32 {
        let d: bool = a == c;
        let hej: bool = ((1+3) == 4) == true;
        if b && d == true {
            return 0;
        };
    }";
    let tree = parse_keyword(function);
    println!("{:#?}", tree);

    //let a = "1 + 3 + 5 - (5*3);";
    /*     let a = "1 + 3 + 5 + true";
    let tree = parse_right_expr(a).unwrap().1;
    let intr = eval_bin_tree(tree);
    println!("{:#?}", intr); */

    /*     let test = "if true {
        let a: i32 = 0;
    }";

    let tree = parse_keyword(test);
    println!("{:#?}", tree); */
}
