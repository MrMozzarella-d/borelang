use std::collections::HashMap;
use std::rc::Rc;
use crate::error::RuntimeErrorType;
use crate::interpreter::{Type, TypeRule, Value};

fn _tostr(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let this = &args[0];
    if let Value::Int(v) = this {
        Ok(Value::Str(Rc::new(format!("{}", v)))).into()
    } else { unreachable!() }
}

pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    let mut vtable = HashMap::new();
    vtable.insert("str".to_string(), Value::RustFunction {
        func: _tostr,
        params: vec![],
        ret_type: TypeRule::Explicit(Type::Str),
    });
    vtables.insert(Type::Float, vtable);
}