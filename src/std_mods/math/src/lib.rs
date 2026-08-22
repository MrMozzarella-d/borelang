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