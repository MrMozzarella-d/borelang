pub mod error;
pub mod interpreter;
mod builtins;

use crate::Value;
pub struct Module {
    pub obj: Value,
}