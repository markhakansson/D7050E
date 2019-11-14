use crate::ast::{
    Value::{Bool, Num, Var},
    *,
};
use crate::interpreter::{EvalErr, EvalRes};
use crate::type_checker::{TypeErr, TypeRes};
use std::collections::HashMap;

pub type Scope<T> = HashMap<String, T>;
pub type Context<T> = Vec<Scope<T>>; // Context is a stack of scopes

pub trait ContextMethods<T, U> {
    fn update_var(&mut self, key: &str, val: &T) -> U;
    fn drop_current_scope(&mut self);
    fn get_val(&mut self, key: &str) -> U;
    fn insert_to_current_scope(&mut self, key: &str, val: &T);
    fn new_scope(&mut self);
}

impl ContextMethods<Value, EvalRes<Value>> for Context<Value> {
    fn update_var(&mut self, key: &str, val: &Value) -> EvalRes<Value> {
        for scope in self.iter_mut().rev() {
            match scope.get(key) {
                Some(_) => {
                    scope.insert(key.to_string(), val.clone());
                    return Ok(val.clone());
                }
                None => continue,
            }
        }

        Err(EvalErr::NotFound("Value not found in context.".to_string()))
    }

    fn drop_current_scope(&mut self) {
        self.pop();
    }

    fn get_val(&mut self, key: &str) -> EvalRes<Value> {
        let mut val_res: EvalRes<Value> = Err(EvalErr::NotFound(
            "Key not found in context scopes".to_string(),
        ));

        for scope in self.iter().rev() {
            match scope.get(key) {
                Some(value) => {
                    val_res = Ok(value.clone());
                    break;
                }
                None => continue,
            };
        }

        val_res
    }

    fn insert_to_current_scope(&mut self, key: &str, val: &Value) {
        let scope_opt = self.last_mut();
        match scope_opt {
            Some(scope) => scope.insert(key.to_string(), val.clone()),
            None => panic!("There are no scopes in the context."),
        };
    }

    fn new_scope(&mut self) {
        let scope: Scope<Value> = HashMap::new();
        self.push(scope);
    }
}

impl ContextMethods<Type, TypeRes<Type>> for Context<Type> {
    fn update_var(&mut self, key: &str, val: &Type) -> TypeRes<Type> {
        for scope in self.iter_mut().rev() {
            match scope.get(key) {
                Some(_) => {
                    scope.insert(key.to_string(), val.clone());
                    return Ok(val.clone());
                }
                None => continue,
            }
        }

        Err(TypeErr(format!(
            "cannot find value ´{}´ in this scope",
            key
        )))
    }

    fn drop_current_scope(&mut self) {
        self.pop();
    }

    fn get_val(&mut self, key: &str) -> TypeRes<Type> {
        for scope in self.iter().rev() {
            match scope.get(key) {
                Some(value) => {
                    return Ok(value.clone());
                }
                None => continue,
            };
        }

        Err(TypeErr(format!(
            "cannot find value ´{}´ in this scope",
            key
        )))
    }

    fn insert_to_current_scope(&mut self, key: &str, val: &Type) {
        let scope_opt = self.last_mut();
        match scope_opt {
            Some(scope) => scope.insert(key.to_string(), val.clone()),
            None => panic!("There are no scopes in the context."),
        };
    }

    fn new_scope(&mut self) {
        let scope: Scope<Type> = HashMap::new();
        self.push(scope);
    }
}

pub type ContextStack<'a, T> = Vec<Context<T>>; // ContextStack is a stack of contexts

pub trait ContextStackMethods<'a, T, U> {
    fn drop_current_context(&'a mut self);
    fn get_last_context(&'a mut self) -> U;
    fn new_context(&'a mut self) -> U;
}

impl<'a> ContextStackMethods<'a, Type, TypeRes<&'a mut Context<Type>>> for ContextStack<'a, Type> {
    fn drop_current_context(&mut self) {
        self.pop();
    }

    fn get_last_context(&'a mut self) -> TypeRes<&'a mut Context<Type>> {
        match self.last_mut() {
            Some(context) => Ok(context),
            None => Err(TypeErr("No context found in FnContext.".to_string())),
        }
    }

    fn new_context(&'a mut self) -> TypeRes<&'a mut Context<Type>> {
        self.push(Context::new());
        self.get_last_context()
    }
}

impl<'a> ContextStackMethods<'a, Value, EvalRes<&'a mut Context<Value>>>
    for ContextStack<'a, Value>
{
    fn drop_current_context(&mut self) {
        self.pop();
    }

    fn get_last_context(&'a mut self) -> EvalRes<&'a mut Context<Value>> {
        match self.last_mut() {
            Some(context) => Ok(context),
            None => Err(EvalErr::NotFound(
                "No context found in FnContext.".to_string(),
            )),
        }
    }

    fn new_context(&'a mut self) -> EvalRes<&'a mut Context<Value>> {
        self.push(Context::new());
        self.get_last_context()
    }
}

pub trait FunctionsMethods<T> {
    fn get_fn(&mut self, name: String) -> T;
}

impl FunctionsMethods<EvalRes<Function>> for Functions {
    fn get_fn(&mut self, name: String) -> EvalRes<Function> {
        for func in self.iter() {
            if func.name == name {
                return Ok(func.clone());
            }
        }
        Err(EvalErr::NotFound("Function not found in tree.".to_string()))
    }
}

impl FunctionsMethods<TypeRes<Function>> for Functions {
    fn get_fn(&mut self, name: String) -> TypeRes<Function> {
        for func in self.iter() {
            if func.name == name {
                return Ok(func.clone());
            }
        }
        Err(TypeErr(format!("function ´{}´ not found", name)))
    }
}
