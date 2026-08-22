use borelang::{Type, Value, Module};

#[unsafe(no_mangle)]
pub extern "C" fn init_module() -> Module {
    let mut module = Module::new("math");
    module.add_fn(
        "abs", _abs,
        vec![
            Type::OneOf(vec![Type::Int, Type::Float])
        ],
        Type::OneOf(vec![Type::Int, Type::Float]),
    );
    module.add_fn(
        "floor", _floor,
        vec![Type::Float],
        Type::Float,
    );
    module.add_fn(
        "ceil", _ceil,
        vec![Type::Float],
        Type::Float,
    );
    module.add_fn(
        "sin", _sin,
        vec![Type::Float],
        Type::Float,
    );
    module.add_fn(
        "cos", _cos,
        vec![Type::Float],
        Type::Float,
    );

    module.add_primitive(
        "PI", Value::Float(std::f64::consts::PI),
    );
    module.add_primitive(
        "EULER", Value::Float(std::f64::consts::E),
    );

    module
}
pub fn _abs(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.abs()),
        Value::Int(x) => Value::Int(x.abs()),
        _ => unreachable!()
    }
}
pub fn _floor(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.floor()),
        _ => unreachable!()
    }
}
pub fn _ceil(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.ceil()),
        _ => unreachable!()
    }
}
pub fn _sin(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.sin()),
        _ => unreachable!()
    }
}
pub fn _cos(args: Vec<Value>) -> Value {
    match args[0] {
        Value::Float(x) => Value::Float(x.cos()),
        _ => unreachable!()
    }
}