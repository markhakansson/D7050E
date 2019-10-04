extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

use std::collections::HashMap;

fn main() {
    // Function declaration 1
    let function = "
    fn func(a: i32, b: bool, c :i32) -> i32 {
        let d: bool = a == c;
        let hej: bool = ((1+3) == 4) == true;
        if d && hej == true {
            return 0;
        };
        return 1;
    }

    fn main(int: i32) -> i32 {
            let a: i32 = func(int, true, int);
            return a;
    }
    ";
    let tree = parse_program(function).unwrap().1;
    println!("{:#?}", tree); 

    // Function declaration 2
/*     let function = "
        fn func(a: i32, b: bool, c :i32) -> () {
            let a: i32 = 3;
            let b: i32 = 17 + a;
        }";
    let tree = parse_program(function).unwrap().1;
    println!("{:#?}", tree); */

    // Function declaration + call
    /*     let function = "
        fn func2(int: i32) -> i32 {
            let a: i32 = func(int, true, int);
            return a;
        }
    ";
    let tree = parse_keyword(function).unwrap().1;
    println!("{:#?}", tree); */

    // Eval bin expression
    /*     let a = "let d: i32 = 1 + a + 5 / 5;";
    let tree = parse_declaration(a).unwrap().1;
    println!("{:#?}", tree);
    let intr = test_eval(tree); */

    // Eval block
    /*     let block = "{
        let a: i32 = 3;
        let b: i32 = a + 17;
    };";
    let tree = parse_block(block).unwrap().1;
    println!("{:#?}", tree);
    let mut scope: Scope = HashMap::new();
    let mut context: Context = vec![];
    context.push(scope);
    let intr = eval_block(tree, &mut context);
    println!("{:#?}", context); */

 /*    let if_st = "{
        let a: bool = true;
        let b: i32 = 5;
        if a {
            let b: i32 = 0;
            let k: i32 = 500;
            if b == 0 {
                let f: bool = true;
                if f {
                    b = 1000;
                };
            };
        }; 

    }";
    let if_tree = parse_block(if_st).unwrap().1;
    println!("{:#?}", if_tree);
    let mut context = Context::new();
    let intr = eval_block(if_tree, &mut context);
    println!("{:#?}", context);  */


}
