extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;

use std::collections::HashMap;

fn main() {
    // Function declaration 1 parser
    let function = "
    fn func(a: i32, b: bool, c :i32) -> i32 {
        let hej: bool = (a == c) && b;
        if hej == true {
            return 0;
        };
        return 1;
    }

    fn test(i: i32) -> () {
        let a: i32 = i + 10;
    }

    fn main() -> () {
            let tjena: i32 = 5;
            let a: i32 = func(tjena, false, 5);
            test(5);
            return a;
    }
    ";
    let mut tree = parse_program(function).unwrap().1;
    let tree_eval = eval_program(&mut tree);
    println!("Tree: {:#?}", tree); 
    println!("Tree_eval: {:#?}", tree_eval); 


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
    let tree = Block::new(parse_block(block).unwrap().1);
    println!("{:#?}", tree);
    let mut fn_context = FnContext::new();
    fn_context.new_context();
    let mut fns = Functions::new();
    let intr = eval_block(tree, &fns, &mut fn_context);
    println!("{:#?}, {:#?}, {:#?}",fn_context, fns, intr); */

/*     let if_st = "{
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
    let if_tree = Block::new(parse_block(if_st).unwrap().1);
    println!("{:#?}", if_tree);
    let mut fn_context = FnContext::new();
    fn_context.new_context();
    let mut fns = Functions::new();
    let intr = eval_block(if_tree, &mut fns, &mut fn_context);
    println!("{:#?}", fn_context); */

    // Function call 1 for interpreter
/*     let main = "
        fn test(i: i32) -> () {
            let a: i32 = 3 + i;
        }

        fn main() -> () {
            let a: i32 = 300;
            let b: bool = true;
            test(a);
        }
    ";
    let mut main_tree = parse_program(main).unwrap().1;
    let main_cntx = eval_program(&mut main_tree).unwrap();
    println!("{:#?}", main_cntx); */

}
