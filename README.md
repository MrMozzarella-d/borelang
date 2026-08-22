# Borelang
## Interpreted, high-level language written completely in Rust
> **NOTE**: Still in very early development, this might change a lot.


## Design Goals
* Rust-like syntax
* High-level semantics and expressiveness
* GDScript-inspired voluntary type safety
* A module system that makes it easy to write code for Bore in Rust

## Current Features
* Constant and Mutable Variables
* 'if', 'for' and 'while' statements
* Lightweight built-ins library (not fully)
* Basic error handler
* Explicit/Any types
* Imports from Rust -> Bore

## Code Example
```
# prints "Hello, world!"
greet("world")

fn greet(who: str) {
  println("Hello, ", who, "!")
}
```

## Devlogs / Notes
* **Contributions**: Currently not accepting outside pull requests, though feel free to suggest features!
 
## Rust Modules
Bore modules can be easily written in Rust and imported into Bore, 
External modules are loaded from
```%localappdata%\borelang\modules\```.      
For example, a simple ```math``` module:
```
use borelang::{Type, TypeRule, Value, Module};

#[unsafe(no_mangle)]
pub extern "C" fn init_module() -> Module {
    let mut module = Module::new("math");
    module.add_fn(
        "abs",
        _abs,
        vec![TypeRule::Explicit(Type::Float)],
        TypeRule::Explicit(Type::Float),
    );
    module.add_primitive(
        "PI",
        Value::Float(std::f64::consts::PI),
    );
    module.add_primitive(
        "EULER",
        Value::Float(std::f64::consts::E),
    );

    module
}
pub fn _abs(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.abs()),
        _ => unreachable!()
    }
}
´´´
