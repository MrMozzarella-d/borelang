use std::collections::HashMap;
use crate::interpreter::{Type, Value};

mod int;
mod float;

pub fn register(vtables: &mut HashMap<Type, HashMap<String, Value>>) {
    int::register(vtables);
    float::register(vtables);
}