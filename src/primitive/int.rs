use std::collections::HashMap;
use std::rc::Rc;
use crate::runtime::{ interpreter::{Type, Value} };

fn _tostr(args: Vec<Value>) -> Value {
    let this = &args[0];
    if let Value::Int(v) = this {
        Value::Str(Rc::new(format!("{}", v)))
    } else { unreachable!() }
}

fn _tofloat(args: Vec<Value>) -> Value {
    let this = &args[0];
    if let Value::Int(v) = this {
        Value::Float(*v as f64)
    } else { unreachable!() }
}

pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    let mut vtable = HashMap::new();
    vtable.insert("str".to_string(), Value::RustFunction {
        func: _tostr,
        params: vec![],
        ret_type: Type::Str,
    });
    vtable.insert("float".to_string(), Value::RustFunction {
        func: _tofloat,
        params: vec![],
        ret_type: Type::Float,
    });
    vtables.insert(Type::Int, vtable);
}