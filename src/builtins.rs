use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::error::RuntimeErrorType;
use crate::interpreter::Value;

pub struct Module {
    pub obj: Value,
}

impl Module {
    pub fn new() -> Self {
        let mut fields: HashMap<String, Value> = HashMap::new();
        fields.insert("println".to_string(), Value::RustFunction(println));
        fields.insert("print".to_string(), Value::RustFunction(print));

        Self {
            obj: Value::Object {
                name: "builtins".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            }
        }
    }
}
pub fn println(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    for arg in args {
        match arg {
            Value::Bool(b) => print!("{}", b),
            Value::Int(i) => print!("{}", i),
            Value::Float(f) => print!("{}", f),
            Value::Str(s) => print!("{}", s),
            _ => return Err(RuntimeErrorType::Other("Cannot print type".to_string()))
        }
    }
    println!();
    Ok(Value::Null)
}
pub fn print(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    for arg in args {
        match arg {
            Value::Bool(b) => print!("{}", b),
            Value::Int(i) => print!("{}", i),
            Value::Float(f) => print!("{}", f),
            Value::Str(s) => print!("{}", s),
            _ => return Err(RuntimeErrorType::Other("Cannot print type".to_string()))
        }
    }
    Ok(Value::Null)
}