use crate::ast::*;
use crate::parser::*;

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    FloatPredicate, OptimizationLevel,
};
use std::collections::HashMap;
use std::error::Error;

type ExprFunc = unsafe extern "C" fn() -> i32;

fn main() {}
