use crate::ast::*;
use crate::parser::*;

use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::Module,
    passes::PassManager,
    IntPredicate,
    types::BasicTypeEnum,
    values::{BasicValueEnum, FloatValue, FunctionValue, InstructionValue, IntValue, PointerValue},
    FloatPredicate, OptimizationLevel,
};
use std::collections::HashMap;
use std::error::Error;

type ExprFunc = unsafe extern "C" fn() -> i32;

pub struct Compiler<'a> {
    pub context: &'a Context,
    pub builder: &'a Builder,
    pub module: &'a Module,
    variables: HashMap<String, PointerValue>,
    fn_value_opt: Option<FunctionValue>,
}

impl<'a> Compiler<'a> {
    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue> {
        self.module.get_function(name)
    }

    #[inline]
    fn get_variable(&self, name: &str) -> &PointerValue {
        match self.variables.get(name) {
            Some(var) => var,
            None => panic!(
                "Could not find a matching variable, {} in {:?}",
                name, self.variables
            ),
        }
    }

    #[inline]
    fn fn_value(&self) -> FunctionValue {
        self.fn_value_opt.unwrap()
    }

    fn compile_expr(&self, expr: Expr) -> IntValue {
        match expr.clone() {
            Expr::Var(var) => {
                let val = self.get_variable(&var);
                self.builder.build_load(*val, &var).into_int_value()
            },
            Expr::Num(i) => self.context.i32_type().const_int(i as u64, false),
            Expr::BinOp(l, op, r) => self.compile_bin_op(*l, op, *r),
            _ => unimplemented!(),
        }

    }     

    fn compile_bin_op(&self, l: Expr, op: Op, r: Expr) -> IntValue {
        let l_val = self.compile_expr(l);
        let r_val = self.compile_expr(r);

        match op {
            Op::BoolOp(token) => self.compile_bool_op(l_val, token, r_val),
            Op::MathOp(token) => self.compile_math_op(l_val, token, r_val),
            Op::RelOp(token) => self.compile_rel_op(l_val, token, r_val),
            _ => panic!("Not a valid expression"),
        }    
    }

    fn compile_bool_op(&self, l: IntValue, token: BoolToken, r: IntValue) -> IntValue {
        match token {
            BoolToken::And => self.builder.build_and(l,r,"and"),
            BoolToken::Or => self.builder.build_or(l,r,"or"),
        }
    }

    fn compile_math_op(&self, l: IntValue, token: MathToken, r: IntValue) -> IntValue {
        match token {
            MathToken::Plus => self.builder.build_int_add(l, r, "add"),
            MathToken::Minus => self.builder.build_int_sub(l, r, "sub"),
            MathToken::Multiply => self.builder.build_int_mul(l, r, "mul"),
            MathToken::Division => self.builder.build_int_signed_div(l, r, "div"),
        }
    }

    fn compile_rel_op(&self, l: IntValue, token: RelToken, r: IntValue) -> IntValue {
        match token {
            RelToken::Equal => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
            RelToken::Ge => self.builder.build_int_compare(IntPredicate::SGT, l, r, "ge"),
            RelToken::Le => self.builder.build_int_compare(IntPredicate::SLT, l, r, "le"),
            RelToken::Neq => self.builder.build_int_compare(IntPredicate::NE, l, r, "neq"),
        }
    }

 /*    fn compile_var_op(&self, l: Expr, op: Op, r: Expr) -> IntValue {
        let l_val = self.compile_expr(l);
        let r_val = self.compile_expr(r);

        match op {
            Op::VarOp(VarToken::PlusEq) => 
        }
    } */

    fn create_entry_block_alloca(&mut self, name: &str) -> PointerValue {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(&entry),
        }
        let alloca = builder.build_alloca(self.context.i32_type(), name);
        self.variables.insert(name.to_string(), alloca);
        alloca
    }

    fn compile_keyword(&mut self, keyword: Expr) -> (InstructionValue, bool) {
        match keyword {
            Expr::Let(var, var_type, expr) => match *var {
                Expr::Var(var) => {
                    let val = self.compile_expr(*expr);
                    let alloca = self.create_entry_block_alloca(&var);
                    let store = self.builder.build_store(alloca, val);

                    (store, false)
                },
                _ => panic!(),
            },
            Expr::Return(expr) => {
                let val = self.compile_expr(*expr);
                (self.builder.build_return(Some(&val)), true)
            },
            _ => unimplemented!(),
        }
    }

    fn compile_block(&mut self, block: Vec<Expr>) -> InstructionValue {
        for expr in block {
            let (cmd, ret) = self.compile_keyword(expr);
            if ret {
                return cmd;
            }
        }
        panic!();
    }

}

pub fn test() {
    let context = Context::create();
    let mut module = context.create_module("expr");
    let builder = context.create_builder();
    let fpm = PassManager::create(&module);
    fpm.initialize();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

    let block = parse_block(
        "{let a: i32 = 7;return a;}"
    ).unwrap().1;

    println!("block {:?}", block);

    let u32_type = context.i32_type();
    let fn_type = u32_type.fn_type(&[], false);
    let function = module.add_function("expr", fn_type, None);
    let basic_block = context.append_basic_block(&function, "entry");
    builder.position_at_end(&basic_block);

    let mut compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        fn_value_opt: Some(function),
        variables: HashMap::new(),
    };

    let res = compiler.compile_block(block);    
    module.print_to_stderr(); 
    let fun_expr: JitFunction<ExprFunc> = 
        unsafe { execution_engine.get_function("expr").ok().unwrap()};

    unsafe {
        println!("{}", fun_expr.call());
    }
    
}

