extern crate d7050e;

use crate::d7050e::ast::*;
use crate::d7050e::interpreter::*;
use crate::d7050e::parser::*;
use crate::d7050e::type_checker::*;

use std::collections::HashMap;

fn main() {
    test();
}
