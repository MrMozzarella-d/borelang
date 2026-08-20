use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::rc::Rc;
use std::sync::Mutex;
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
    let mut lock = _stdout().lock().unwrap();
    for arg in args {
        match arg {
            Value::Bool(b) => { write!(lock, "{} ", b).unwrap(); },
            Value::Int(i) => { write!(lock, "{} ", i).unwrap();  },
            Value::Float(f) => { write!(lock, "{} ", f).unwrap();  },
            Value::Str(s) => { write!(lock, "{} ", s).unwrap();  },
            _ => return Err(RuntimeErrorType::Other("Cannot print type".to_string()))
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
            Value::Bool(b) => { write!(lock, "{} ", b).unwrap(); },
            Value::Int(i) => { write!(lock, "{} ", i).unwrap();  },
            Value::Float(f) => { write!(lock, "{} ", f).unwrap();  },
            Value::Str(s) => { write!(lock, "{} ", s).unwrap();  },
            _ => return Err(RuntimeErrorType::Other("Cannot print type".to_string()))
        }
    }
    lock.flush().unwrap();
    Ok(Value::Null)
}

fn _stdout() -> &'static Mutex<BufWriter<Stdout>> {
    static WRITER: std::sync::OnceLock<Mutex<BufWriter<Stdout>>> = std::sync::OnceLock::new();
    WRITER.get_or_init(|| Mutex::new(BufWriter::new(stdout())))
}