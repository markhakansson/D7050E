extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::llvm;
use crate::d7050e::parser::*;
use crate::d7050e::type_checker::*;

use std::collections::HashMap;

fn main() {
    let program = "
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
    llvm::compile_program(ast_tree.unwrap().1);
}
