use crate::error::RuntimeError;
use crate::interpreter::{RuntimeValue};

fn println(args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    for arg in args {
        match arg {
            RuntimeValue::Str(s) => print!("{}", s),
            RuntimeValue::Int(i) => print!("{}", i),
            RuntimeValue::Float(f) => print!("{}", f),
            RuntimeValue::Bool(b) => print!("{}", b),
            _ => {}
        }
    };
    println!();
    Ok((RuntimeValue::Null))
}
fn print(args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
    for arg in args {
        match arg { 
            RuntimeValue::Str(s) => print!("{}", s),
            RuntimeValue::Int(i) => print!("{}", i),
            RuntimeValue::Float(f) => print!("{}", f),
            RuntimeValue::Bool(b) => print!("{}", b),
            _ => {}
        }
    };
    Ok((RuntimeValue::Null))
}