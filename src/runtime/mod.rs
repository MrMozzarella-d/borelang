pub mod error;
pub mod interpreter;
mod builtins;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::{TypeRule, Value};
pub struct Module {
    pub obj: Value,
}
impl Module {
    pub fn new(name: &str) -> Self {
        Self {
            obj: Value::Object { name: name.to_string(), fields: Rc::new(RefCell::new(HashMap::new())) },
        }
    }
    pub fn add_fn(&mut self, name: &str, func: fn(Vec<Value>) -> Value, params: Vec<TypeRule>, ret_type: TypeRule) -> &mut Self {
        if let Value::Object { fields, .. } = &self.obj {
            let mut borrow = fields.borrow_mut();
            borrow.insert(name.to_string(), Value::RustFunction {
                func,
                params,
                ret_type,
            });
        }
        self
    }
    pub fn add_primitive(&mut self, name: &str, value: Value) -> &mut Self {
        if let Value::Object { fields, .. } = &self.obj {
            let mut borrow = fields.borrow_mut();
            borrow.insert(name.to_string(), value);
        }
        self
    }
}