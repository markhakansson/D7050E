extern crate simple_rustc;

use crate::simple_rustc::ast::*;
use crate::simple_rustc::interpreter::*;
use crate::simple_rustc::llvm;
use crate::simple_rustc::parser::*;
use crate::simple_rustc::type_checker::*;

use std::collections::HashMap;

fn main() {
/*     let program = "
    fn test() -> i32 {
        return 5;
    }
    fn main() -> i32 {
        let i: i32 = 300;
        let b: bool = true;
        let c: i32 = 0;
        test();
        while b {
            c = 1;
            b = false;
        };
        return i;
    }";
    let ast_tree = parse_program(program);
    llvm::compile_program(ast_tree.unwrap().1); */

    let program1 = "
    fn testone(b: bool) -> i32 {
        let a: i32 = 0;
        if b {
            a = 50;
        };
    
        return a;
    }
    
    fn testtwo(a: i32, b: bool, c: i32) -> i32 {
        let variable: bool = (a == c) && b;
        let num: i32 = 0;
        while variable == true {
            num += 1;
            variable = false;
        };
        return num;    
    }
    
    fn main() -> () {
        let a: i32 = testone(true);
        let b: i32 = testtwo(a, true, 50);
        let c: i32 = a + b;
    }"; 

    let mut program_tree = parse_program(program1).unwrap().1;
    println!("{:#?}", program_tree);
    let test = tc_program(&mut program_tree);
    //let test = eval_program(&mut program_tree);
    println!("{:#?}", test);
}
