use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::rc::Rc;
use std::sync::Mutex;
use crate::error::RuntimeErrorType;
use crate::interpreter::{TypeRule, Value};
use crate::interpreter::Type::Null;

pub struct Module {
    pub obj: Value,
}

impl Module {
    pub fn new() -> Self {
        let mut fields: HashMap<String, Value> = HashMap::new();
        fields.insert("println".to_string(), Value::RustFunction {
            func: println, 
            params: vec![TypeRule::Any], 
            ret_type: TypeRule::Explicit(Null)
        });
        fields.insert("print".to_string(), Value::RustFunction {
            func: print, 
            params: vec![TypeRule::Any], 
            ret_type: TypeRule::Explicit(Null)
        });
        fields.insert("null".to_string(), Value::Null);
        fields.insert("true".to_string(), Value::Bool(true));
        fields.insert("false".to_string(), Value::Bool(false));

        Self {
            obj: Value::Object {
                name: "builtins".to_string(),
                fields: Rc::new(RefCell::new(fields)),
            }
        }
    }
}
pub fn println(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let mut lock = _stdout().lock().unwrap();
    for arg in args {
        match arg {
            Value::Bool(b) => write!(lock, "{}", b).unwrap(),
            Value::Int(i) => write!(lock, "{}", i).unwrap(),
            Value::Float(f) => write!(lock, "{}", f).unwrap(),
            Value::Str(s) => write!(lock, "{}", s).unwrap(),
            other => write!(lock, "{:?}", other).unwrap(),
        }
    }
    writeln!(lock).unwrap();
    lock.flush().unwrap();
    Ok(Value::Null)
}
pub fn print(args: Vec<Value>) -> Result<Value, RuntimeErrorType> {
    let mut lock = _stdout().lock().unwrap();
    for arg in args {
        match arg {
            Value::Bool(b) => write!(lock, "{}", b).unwrap(),
            Value::Int(i) => write!(lock, "{}", i).unwrap(),
            Value::Float(f) => write!(lock, "{}", f).unwrap(),
            Value::Str(s) => write!(lock, "{}", s).unwrap(),
            other => write!(lock, "{:?}", other).unwrap(),
        }
    }
    lock.flush().unwrap();
    Ok(Value::Null)
}

fn _stdout() -> &'static Mutex<BufWriter<Stdout>> {
    static WRITER: std::sync::OnceLock<Mutex<BufWriter<Stdout>>> = std::sync::OnceLock::new();
    WRITER.get_or_init(|| Mutex::new(BufWriter::new(stdout())))
}