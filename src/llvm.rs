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
    FloatPredicate, IntPredicate, OptimizationLevel,
};
use std::collections::HashMap;

type ExprFunc = unsafe extern "C" fn() -> i32;

pub struct Compiler<'a> {
    pub context: &'a Context,
    pub builder: &'a Builder,
    pub module: &'a Module,
    execution_engine: &'a ExecutionEngine,
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
            }
            Expr::Num(i) => self.compile_num(i),
            Expr::Bool(b) => {
                if b {
                    self.context.bool_type().const_int(1, false)
                } else {
                    self.context.bool_type().const_int(0, false)
                }
            }
            Expr::BinOp(l, op, r) => self.compile_bin_op(*l, op, *r),
            Expr::FuncCall(fn_call) => self.compile_function_call(fn_call),
            _ => unimplemented!(),
        }
    }

    fn compile_num(&self, num: i32) -> IntValue {
        self.context.i32_type().const_int(num as u64, false)
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
            BoolToken::And => self.builder.build_and(l, r, "and"),
            BoolToken::Or => self.builder.build_or(l, r, "or"),
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
            RelToken::Ge => self
                .builder
                .build_int_compare(IntPredicate::SGT, l, r, "ge"),
            RelToken::Le => self
                .builder
                .build_int_compare(IntPredicate::SLT, l, r, "le"),
            RelToken::Neq => self
                .builder
                .build_int_compare(IntPredicate::NE, l, r, "neq"),
        }
    }

    // Kind of hacked together but I did not want to spend to much time on this part
    fn compile_var_op(&self, var: Expr, op: Op, expr: Expr) -> InstructionValue {
        let val = self.compile_expr(expr);

        match op {
            Op::VarOp(VarToken::Assign) => {
                let var_ptr = match var {
                    Expr::Var(var) => self.get_variable(&var),
                    _ => panic!(),
                };
                self.builder.build_store(*var_ptr, val)
            }
            Op::VarOp(VarToken::PlusEq) => {
                let var_ptr = match &var {
                    Expr::Var(var) => self.get_variable(&var),
                    _ => panic!(),
                };
                let var_val = self.compile_expr(var);
                let new_val = self.compile_math_op(var_val, MathToken::Plus, val);
                self.builder.build_store(*var_ptr, new_val)
            }
            Op::VarOp(VarToken::MinEq) => {
                let var_ptr = match &var {
                    Expr::Var(var) => self.get_variable(&var),
                    _ => panic!(),
                };
                let var_val = self.compile_expr(var);
                let new_val = self.compile_math_op(var_val, MathToken::Minus, val);
                self.builder.build_store(*var_ptr, new_val)
            }
            Op::VarOp(VarToken::MulEq) => {
                let var_ptr = match &var {
                    Expr::Var(var) => self.get_variable(&var),
                    _ => panic!(),
                };
                let var_val = self.compile_expr(var);
                let new_val = self.compile_math_op(var_val, MathToken::Multiply, val);
                self.builder.build_store(*var_ptr, new_val)
            }
            _ => unimplemented!(),
        }
    }

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

    fn compile_function_call(&self, fn_call: FunctionCall) -> IntValue {
        let function = self.module.get_function(&fn_call.name).unwrap();
        let args: Vec<BasicValueEnum> = fn_call
            .args
            .content
            .iter()
            .map(|a| self.compile_expr(a.clone()).into())
            .collect();
        let call = self.builder.build_call(function, &args, &fn_call.name);
        *call.try_as_basic_value().left().unwrap().as_int_value()
    }

    fn compile_keyword(&mut self, keyword: Expr) -> (InstructionValue, bool) {
        match keyword.clone() {
            Expr::Let(var, _, expr) => match *var {
                Expr::Var(var) => {
                    let val = self.compile_expr(*expr);
                    let alloca = self.create_entry_block_alloca(&var);
                    let store = self.builder.build_store(alloca, val);

                    (store, false)
                }
                _ => panic!(),
            },
            Expr::VarOp(var, op, expr) => (self.compile_var_op(*var, op, *expr), false),
            Expr::If(cond, block) => (self.compile_if(*cond, block), false),
            Expr::While(cond, block) => (self.compile_while(*cond, block), false),
            Expr::Return(expr) => {
                let val = self.compile_expr(*expr);
                (self.builder.build_return(Some(&val)), true)
            }
            Expr::FuncCall(_) => (self.compile_expr(keyword).as_instruction().unwrap(), false),
            _ => unimplemented!(),
        }
    }

    fn compile_if(&mut self, condition: Expr, block: Block) -> InstructionValue {
        let cond = self.compile_expr(condition);
        let then_block = self.context.append_basic_block(&self.fn_value(), "then");
        let cont_block = self.context.append_basic_block(&self.fn_value(), "cont");

        self.builder
            .build_conditional_branch(cond, &then_block, &cont_block);
        self.builder.position_at_end(&then_block);
        self.compile_block(block);

        self.builder.build_unconditional_branch(&cont_block);
        self.builder.position_at_end(&cont_block);

        let phi = self.builder.build_phi(self.context.i32_type(), "iftmp");
        phi.add_incoming(&[
            (&self.compile_num(0), &then_block),
            (&self.compile_num(0), &cont_block),
        ]);
        phi.as_instruction()
    }

    fn compile_while(&mut self, condition: Expr, block: Block) -> InstructionValue {
        let do_block = self.context.append_basic_block(&self.fn_value(), "do");
        let cont_block = self.context.append_basic_block(&self.fn_value(), "cont");

        self.builder.build_conditional_branch(
            self.compile_expr(condition.clone()),
            &do_block,
            &cont_block,
        );
        self.builder.position_at_end(&do_block);
        self.compile_block(block);

        self.builder.build_conditional_branch(
            self.compile_expr(condition.clone()),
            &do_block,
            &cont_block,
        );
        self.builder.position_at_end(&cont_block);

        // This phi node does nothing, used to return an InstructionValue
        let phi = self.builder.build_phi(self.context.i32_type(), "whiletmp");
        phi.add_incoming(&[
            (&self.compile_num(0), &do_block),
            (&self.compile_num(0), &do_block),
        ]);
        phi.as_instruction()
    }

    fn compile_block(&mut self, block: Block) -> InstructionValue {
        let mut last_cmd: Option<InstructionValue> = None;
        for expr in block.content {
            let (cmd, ret) = self.compile_keyword(expr);
            if ret {
                return cmd;
            }
            last_cmd = Some(cmd);
        }

        match last_cmd {
            Some(instruction) => instruction,
            None => panic!(),
        }
    }

    // Still working on compiling parameters
    fn compile_function(&self, func: Function) -> FunctionValue {
        let param_types: Vec<BasicTypeEnum> = func
            .params
            .iter()
            .map(|param| match param.param_type {
                Type::Int32 => self.context.i32_type().into(),
                Type::Bool => self.context.bool_type().into(),
                _ => unreachable!(),
            })
            .collect();

        let fn_ret_type = match func.return_type {
            Type::Bool => self.context.bool_type().fn_type(&param_types, false),
            Type::Int32 => self.context.i32_type().fn_type(&param_types, false),
            Type::Void => self.context.void_type().fn_type(&param_types, false),
        };

        self.module.add_function(&func.name, fn_ret_type, None)
    }
}

pub fn compile_program(fn_list: Functions) {
    let context = Context::create();
    let mut module = context.create_module("llvm-program");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let mut compiler = Compiler {
        context: &context,
        builder: &builder,
        module: &module,
        execution_engine: &execution_engine,
        fn_value_opt: None,
        variables: HashMap::new(),
    };

    for function in fn_list {
        let llvm_func = compiler.compile_function(function.clone());
        compiler.fn_value_opt = Some(llvm_func);
        let basic_block = compiler.context.append_basic_block(&llvm_func, "entry");

        compiler.builder.position_at_end(&basic_block);
        compiler.compile_block(function.block);
    }

    module.print_to_stderr();
    let fun_expr: JitFunction<ExprFunc> =
        unsafe { execution_engine.get_function("main").ok().unwrap() };

    unsafe {
        println!("{}", fun_expr.call());
    }
}

pub fn test() {
    let context = Context::create();
    let mut module = context.create_module("expr");
    let builder = context.create_builder();
    let fpm = PassManager::create(&module);
    fpm.initialize();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    let block = parse_block(
        //"{let a: i32 = 5;let b: i32 = 0; let c: bool = b > a; return c;}"
        //"{let b: bool = false; if b {return 5;}; return 4;}"
        "{let b: bool = true; let i: i32 = 0; while b {i = 1; b = false;}; return i;}",
    )
    .unwrap()
    .1;

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
        execution_engine: &execution_engine,
        fn_value_opt: Some(function),
        variables: HashMap::new(),
    };

    let res = compiler.compile_block(Block::new(block));
    module.print_to_stderr();
    let fun_expr: JitFunction<ExprFunc> =
        unsafe { execution_engine.get_function("expr").ok().unwrap() };

    unsafe {
        println!("{}", fun_expr.call());
    }
}
