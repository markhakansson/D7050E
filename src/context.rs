use ast::Value::{Bool,Var,Num};
use std::collections::HashMap;

pub type Scope = HashMap<Value, Value>; 
pub type Context = Vec<Scope>; // Context is a stack of scopes
pub type FnContext = Vec<Context>; // FnContext is a stack of scopes


