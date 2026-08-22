use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::rc::Rc;
use std::sync::Mutex;

use crate::Module;
use crate::runtime::interpreter::{Value, Type};

pub fn init_module() -> Module {
    let mut fields: HashMap<String, Value> = HashMap::new();
    fields.insert("println".to_string(), Value::RustFunction {
        func: _println,
        params: vec![Type::Any],
        ret_type: Type::Null
    });
    fields.insert("print".to_string(), Value::RustFunction {
        func: _print,
        params: vec![Type::Any],
        ret_type: Type::Null
    });
    fields.insert("null".to_string(), Value::Null);
    fields.insert("true".to_string(), Value::Bool(true));
    fields.insert("false".to_string(), Value::Bool(false));

    let module = Module {
        obj: Value::Object {
            name: "builtins".to_string(),
            fields: Rc::new(RefCell::new(fields)),
        }
    };
    module
}
pub fn _println(args: Vec<Value>) -> Value {
    let mut lock = __stdout().lock().unwrap();
    for arg in args {
        write!(lock, "{}", arg).unwrap();
    }
    writeln!(lock).unwrap();
    lock.flush().unwrap();
    Value::Null
}
pub fn _print(args: Vec<Value>) -> Value {
    let mut lock = __stdout().lock().unwrap();
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
    Value::Null
}

fn __stdout() -> &'static Mutex<BufWriter<Stdout>> {
    static WRITER: std::sync::OnceLock<Mutex<BufWriter<Stdout>>> = std::sync::OnceLock::new();
    WRITER.get_or_init(|| Mutex::new(BufWriter::new(stdout())))
}