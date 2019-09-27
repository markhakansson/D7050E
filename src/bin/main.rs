extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

fn main() {
/*     let function = "
    fn func(a: i32, b: bool, c :i32) -> i32 {
        let d: bool = a == c;
        let hej: bool = ((1+3) == 4) == true;
        if b && d == true {
            return 0;
        };
    }";
    let tree = parse_keyword(function);
    println!("{:#?}", tree); */

    //let a = "1 + 3 + 5 - (5*3);";
    //let a = "1 + 3 + 5 + true";
    let a = "let d: i32 = 1 + 3 + 5 / 5;";
    let tree = parse_keyword(a).unwrap().1;
    println!("{:#?}",tree);
    let intr = test_eval(tree);
    println!("{:#?}", intr);

    /*     let test = "if true {
        let a: i32 = 0;
    }";

    let tree = parse_keyword(test);
    println!("{:#?}", tree); */
}
