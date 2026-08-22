use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env::{var};
use std::fmt::{write, Display, Formatter};
use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::PathBuf;
use std::rc::Rc;
use libloading::{Library, Symbol};
use crate::runtime::{builtins, error};
use error::{RuntimeError, RuntimeErrorType};
use crate::{primitive, Module};

use crate::syntax::node::{Statement, StatementType, ExpressionType, Expression};
use crate::syntax::token::TokenData;

type Env = Rc<RefCell<Environment>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    UInt(u128),
    Int(i64),
    Float(f64),
    Str(Rc<String>),
    Bool(bool),
    Array(Rc<RefCell<Vec<(Value, Type)>>>),
    Object {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
    BoreFunction(Rc<Statement>),
    RustFunction {
        func: fn(args: Vec<Value>) -> Value,
        params: Vec<Type>,
        ret_type: Type,
    },
    BoundMethod {
        this: Box<Value>,
        func: Box<Value>,
    },
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Any,
    UInt,
    Int,
    Float,
    Str,
    Bool,
    Null,
    Array(Box<Type>),
    Class(String),
    Function {
        params: Vec<Type>,
        returns: Box<Type>,
    },
    OneOf(Vec<Type>),
    Unresolved(String),
}
impl Type {
    pub fn accepts(&self, value: &Value) -> bool {
        if self == &Type::UInt && let Value::Int(num) = value {
            return *num >= 0;
        }

        match (self, value.to_type()) {
            (Type::Any, _) => true,
            (Type::OneOf(v), other) => v.contains(&other),
            (Type::Array(a), Type::Array(b)) => a.accepts_t(&b),
            (a, b) => a == &b,
        }
    }
    pub fn accepts_t(&self, ty: &Type) -> bool {
        match (self, ty) {
            (Type::Any, _) => true,
            (Type::OneOf(v), other) => v.contains(&other),
            (Type::Array(a), Type::Array(b)) => a.accepts_t(&b),
            (a, b) => a == b,
        }
    }
}
impl Value {
    pub fn to_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::UInt(_) => Type::UInt,
            Value::Float(_) => Type::Float,
            Value::Str(_) => Type::Str,
            Value::Bool(_) => Type::Bool,
            Value::Null => Type::Null,
            Value::Array(rc) => {
                let borrow = rc.borrow();
                if borrow.is_empty() {
                    Type::Array(Box::new(Type::Any))
                } else {
                    let ty = &borrow.get(0).unwrap().1;
                    Type::Array(Box::new(ty.clone()))
                }
            },
            Value::Object { name, .. } => Type::Class(name.clone()),
            Value::BoreFunction(stmt) => {
                match stmt.statement_type {
                    StatementType::FunctionDeclaration { name: _, ref params, body: _, ref rtn} => {
                        let mut param_rules = Vec::new();
                        for p in params {
                            param_rules.push(p.1.clone());
                        };
                        Type::Function {
                            params: param_rules,
                            returns: Box::new(rtn.clone()),
                        }
                    },
                    _ => Type::Function {
                        params: Vec::new(),
                        returns: Box::new(Type::Any),
                    }
                }
            }
            Value::BoundMethod { this: _, func } => {
                match &**func {
                    Value::RustFunction { func: _, params, ret_type } => {
                        let mut param_rules = Vec::new();
                        for p in params {
                            param_rules.push(p.clone());
                        };
                        Type::Function {
                            params: param_rules,
                            returns: Box::new(ret_type.clone()),
                        }
                    },
                    Value::BoreFunction( stmt ) => {
                        match stmt.statement_type {
                            StatementType::FunctionDeclaration { name: _, ref params, body: _, ref rtn} => {
                                let mut param_rules = Vec::new();
                                for p in params {
                                    param_rules.push(p.1.clone());
                                };
                                Type::Function {
                                    params: param_rules,
                                    returns: Box::new(rtn.clone()),
                                }
                            },
                            _ => Type::Function {
                                params: Vec::new(),
                                returns: Box::new(Type::Any),
                            }
                        }
                    },
                    _ => Type::Function {
                        params: Vec::new(),
                        returns: Box::new(Type::Any),
                    }
                }
            }
            Value::RustFunction { func: _, params, ret_type } => {
                Type::Function {
                    params: params.clone(),
                    returns: Box::new(ret_type.clone()),
                }
            }
        }
    }
    pub fn matches(&self, expected: &Type, line: usize, column: usize) -> Result<(), RuntimeError> {
        if expected.accepts(self) {
            Ok(())
        } else {
            Err(RuntimeError {
                error_type: RuntimeErrorType::TypeMismatch(
                    expected.clone(),
                    self.to_type(),
                ),
                line,
                column,
            })
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(num) => write!(f, "{}", num),
            Value::UInt(num) => write!(f, "{}", num),
            Value::Float(num) => write!(f, "{}", num),
            Value::Str(str) => write!(f, "{}", str),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Array(s) => {
                write!(f, "[{}]",
                       s.borrow()
                           .iter()
                           .map(|x| {
                               let v = &x.0;
                               return if let Value::Str(s) = v {
                                   format!("\"{}\"", s)
                               } else {
                                   v.to_string()
                               }
                           } )
                           .collect::<Vec<_>>()
                           .join(", "))
            },
            o => write!(f, "{:?}", o),
        }
    }
}
#[derive(Clone, Debug)]
pub struct Variable {
    pub is_mutable: bool,
    pub ty: Type,
    pub value: Value,
}
#[derive(Clone, Debug)]
pub struct Environment {
    vars: HashMap<String, Variable>,
    //types: HashMap<String, Type>,
    enclosing: Option<Env>
}
impl Environment {
    pub fn new(enclosing: Option<Env>) -> Self {
        Self {
            vars: HashMap::new(),
            //types: HashMap::new(),
            enclosing,
        }
    }
    pub fn define(&mut self, name: &String, var: Variable) -> Result<&mut Environment, RuntimeErrorType> {
        if self.vars.contains_key(name) {
            return Err(RuntimeErrorType::VariableAlreadySet(name.clone()));
        }
        self.vars.insert(name.clone(), var);
        Ok(self)
    }
    pub fn get(&self, name: &String) -> Result<Variable, RuntimeErrorType> {
        if let Some(value) = self.vars.get(name) {
            return Ok(value.clone());
        }
        if let Some(ref enc) = self.enclosing {
            let borrow = enc.borrow();
            let v = borrow.get(name);
            return v;
        }
        Err(RuntimeErrorType::VariableNotFound(name.clone()))
    }
    pub fn set(&mut self, name: &String, new: &Value, user: bool) -> Result<&mut Environment, RuntimeErrorType> {
        if let Some(var) = self.vars.get_mut(name) {
            if !var.is_mutable && user {
                return Err(RuntimeErrorType::AttemptToChangeConstantVar(name.clone()))
            }
            if var.ty != Type::Any && var.ty != new.to_type() {
                return Err(RuntimeErrorType::TypeMismatch(var.ty.clone(), new.to_type().clone()))
            }
            var.value = new.clone();
        } else {
            if let Some(ref enclosing) = self.enclosing {
                let mut borrow = enclosing.borrow_mut();
                borrow.set(name, new, user)?;
            } else {
                return Err(RuntimeErrorType::VariableNotFound(name.clone()))
            }
        }
        Ok(self)
    }
}
pub struct Interpreter {
    ast: Vec<Statement>,
    vtables: HashMap<Type, HashMap<String, Value>>,
    lib_cache: Vec<Library>
}
impl Interpreter {
    pub fn new(ast: Vec<Statement>) -> Self {
        Self {
            ast,
            vtables: HashMap::new(),
            lib_cache: Vec::new(),
        }
    }
    pub fn import_mod(&mut self, name: &String, env: &Env) -> Result<(), RuntimeErrorType> {
        if let Ok(_) = env.borrow().get(name) {
            return Err(RuntimeErrorType::VariableAlreadySet(name.clone()))
        };
        let file_n = if cfg!(target_os = "windows") {
            format!("{}.dll", name)
        } else if cfg!(target_os = "macos") {
            format!("{}.dylib", name)
        } else {
            format!("{}.so", name)
        };

        let std = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("std_mods")
            .join(&file_n);
        let sysdir = if cfg!(target_os = "windows") {
            let path = var("LOCALAPPDATA")
                .unwrap_or_else(|_| {
                    let home = var("USERPROFILE").unwrap_or_else(|_| "C:\\".to_string());
                    format!("{}\\Appdata\\Local", home)
                });
            PathBuf::from(path).join("borelang").join("modules")
        } else {
            PathBuf::from("/usr/lib/borelang")
        };
        if !sysdir.exists() {
            _ = create_dir_all(&sysdir);
        }
        let sys = sysdir.join(&file_n);
        let path = if std.exists() {
            std
        } else if sys.exists() {
            sys
        } else {
            return Err(RuntimeErrorType::ModuleNotFound(name.clone()));
        };

        unsafe {
            let lib = Library::new(&path)
                .map_err(|_| RuntimeErrorType::ModuleNotFound(name.clone()))?;
            let search_module: Symbol<extern "C" fn() -> Module> = lib.get(b"init_module")
                .map_err(|_| RuntimeErrorType::ImportedModuleDoesntHaveInitFunction(name.clone()))?;
            let module: Module = search_module();
            if let Value::Object { fields, name } = &module.obj {
                let mut b = env.borrow_mut();
                b.define(name, Variable {
                    is_mutable: false,
                    ty: Type::Class(name.clone()),
                    value: Value::Object { fields: fields.clone(), name: name.clone() },
                })?;
            }
            self.lib_cache.push(lib);
            Ok(())
        }
    }
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        let mut global_env = Environment::new(None);
        // built-ins module
        let builtins = builtins::init_module();
        if let Value::Object { fields, .. } = &builtins.obj {
            for (name, value) in fields.borrow().iter() {
                let var = Variable {
                    is_mutable: false,
                    ty: Type::Any,
                    value: value.clone(),
                };
                let v = global_env.define(name, var);
                self.comply_def_err(v, 1, 0)?;
            }
        }
        // pre-register of all functions
        for stmt in self.ast.iter() {
            if let StatementType::FunctionDeclaration { ref name, ref params, body: _, ref rtn } = stmt.statement_type {
                let func_v = Value::BoreFunction(Rc::new(stmt.clone()));
                let mut param_rules = Vec::new();
                for (_, ty, _) in params {
                    param_rules.push(ty.clone());
                }
                let var = Variable {
                    is_mutable: false,
                    ty: Type::Function {
                        params: param_rules,
                        returns: Box::new(rtn.clone()),
                    },
                    value: func_v,
                };
                let v = global_env.define(name, var);
                self.comply_def_err(v, stmt.line, stmt.column)?;
            }
        }
        // vtables
        primitive::register(&mut self.vtables);

        let rc = Rc::new(RefCell::new(global_env));
        for i in 0..self.ast.len() {
            let stmt = self.ast[i].clone();
            if let Err(e) = self.run_statement(&stmt, &rc) {
                return Err(e);
            }
        }
        Ok(())
    }
    pub fn run_statement(&mut self, stmt: &Statement, env: &Rc<RefCell<Environment>>) -> Result<Option<Value>, RuntimeError> {
        let ty = &stmt.statement_type;
        match ty {
            StatementType::VariableDeclaration { mutable, name, ty, value } => {
                let name = name.to_string();
                if let Ok(_) = env.borrow().get(&name) {
                    return Err(RuntimeError {
                        error_type: RuntimeErrorType::VariableAlreadySet(name),
                        line: stmt.line,
                        column: stmt.column,
                    })
                }
                if let Some(value) = value.clone() { // w/ value
                    let runtime = self.evaluate_expression(&value, env)?;
                    if !ty.accepts(&runtime) {
                        return Err(RuntimeError {
                            error_type: RuntimeErrorType::TypeMismatch(ty.clone(), runtime.to_type()),
                            line: stmt.line,
                            column: stmt.column,
                        })
                    }
                    let var = Variable {
                        is_mutable: *mutable,
                        ty: ty.clone(),
                        value: runtime,
                    };
                    let mut borrow = env.borrow_mut();
                    let v = borrow.define(&name, var);
                    self.comply_def_err(v, value.line, value.column)?;
                } else { // none
                    let var = Variable {
                        is_mutable: *mutable,
                        ty: ty.clone(),
                        value: Value::Null,
                    };
                    let mut borrow = env.borrow_mut();
                    let v = borrow.define(&name, var);
                    self.comply_def_err(v, stmt.line, stmt.column)?;
                }
                Ok(None)
            },
            StatementType::ForLoop { var_decl: _, start: _, end: _, body: _ }
            | StatementType::If { condition: _, then_branch: _, else_branch: _ }
            | StatementType::WhileLoop { condition: _, body: _ } => {
                let v = self.run_body(stmt, env);
                v
            },
            StatementType::Expression(expr) => {
                let v = self.evaluate_expression(&expr, env);
                let result = v.map(|_| None);
                result
            },
            StatementType::Return(rt_v) => {
                if let Some(ret_val) = rt_v {
                    let val = self.evaluate_expression(&ret_val, env)?;
                    Ok(Some(val))
                } else {
                    Ok(Some(Value::Null))
                }
            },
            StatementType::Import { mods } => {
                for module in mods {
                    let v = self.import_mod(module, env);
                    if let Err(error_type) = v {
                        return Err(RuntimeError {
                            error_type,
                            line: stmt.line,
                            column: stmt.column,
                        });
                    }
                }
                Ok(None)
            }
            _ => { Ok(None) }
        }
    }
    
    pub fn evaluate_expression(&mut self, expression: &Expression, env: &Env) -> Result<Value, RuntimeError> {
        match expression.expression_type {
            ExpressionType::BinaryOp{ref left, ref op, ref right} => {
                let left_v = self.evaluate_expression(left, env)?;
                let right_v = self.evaluate_expression(right, env)?;
                match op.token_data {
                    TokenData::Add => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l + r)),
                        (Value::Str(l), Value::Str(r)) => Ok(Value::Str(Rc::new(format!("{}{}", l, r)))),
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Sub => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l - r)),
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Mul => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l * r)),
                        (Value::Str(l), Value::Int(r)) => {
                            let mut str = String::new();
                            for _ in 0..r {
                                str = format!("{}{}", str, l);
                            };
                            Ok(Value::Str(Rc::new(str)))
                        }
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Div => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l / r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Float(l / r)),
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Equivalent => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l == r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l == r)),
                        (Value::Str(l), Value::Str(r)) => Ok(Value::Bool(l == r)),
                        (Value::RustFunction { func: l, .. }, Value::RustFunction { func: r, .. }) =>
                            Ok(Value::Bool(std::ptr::fn_addr_eq(l, r))),
                        (Value::BoreFunction(l), Value::BoreFunction(r)) => Ok(Value::Bool(std::ptr::eq(&l, &r))),
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::NotEqual => match (left_v, right_v) {
                        (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l != r)),
                        (Value::Float(l), Value::Float(r)) => Ok(Value::Bool(l != r)),
                        (Value::Str(l), Value::Str(r)) => Ok(Value::Bool(l != r)),
                        (Value::RustFunction { func: l, .. }, Value::RustFunction { func: r, .. }) =>
                            Ok(Value::Bool(!std::ptr::fn_addr_eq(l, r))),
                        (Value::BoreFunction(l), Value::BoreFunction(r)) => Ok(Value::Bool(!std::ptr::eq(&l, &r))),
                        (l_val, r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::Incompatible(l_val, r_val),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::GreaterThanEqual =>
                        self.compare_values(left_v, right_v, |l, r| l >= r, right.line, right.column),
                    TokenData::GreaterThan =>
                        self.compare_values(left_v, right_v, |l, r| l > r, right.line, right.column),
                    TokenData::LessThan =>
                        self.compare_values(left_v, right_v, |l, r| l < r, right.line, right.column),
                    TokenData::LessThanEqual =>
                        self.compare_values(left_v, right_v, |l, r| l <= r, right.line, right.column),
                    TokenData::And => match (left_v, right_v) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l && r)),
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::CannotOperateOnType("non-bool".to_string(), "&&".to_string()),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Or => match (left_v, right_v) {
                        (Value::Bool(l), Value::Bool(r)) => Ok(Value::Bool(l || r)),
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::CannotOperateOnType("non-bool".to_string(), "??".to_string()),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    _ => Err(RuntimeError {
                        error_type: RuntimeErrorType::UnexpectedToken(op.token_data.clone()),
                        line: op.line,
                        column: op.column,
                    })
                }
            },
            ExpressionType::Unary { ref op, ref right } => {
                let v = self.evaluate_expression(right, env)?;
                let ty = v.to_type();
                match op.token_data {
                    TokenData::Sub => {
                        match v {
                            Value::Int(i) => Ok(Value::Int(-i)),
                            Value::Float(f) => Ok(Value::Float(-f)),
                            _ => Err(RuntimeError {
                                error_type: RuntimeErrorType::AttemptToUseUnaryOnWrongType(op.clone().token_data, ty),
                                line: op.line, column: op.column,
                            })
                        }
                    },
                    TokenData::Not => {
                        if let Value::Bool(old) = v {
                            Ok(Value::Bool(!old))
                        } else {
                            Err(RuntimeError {
                            error_type: RuntimeErrorType::AttemptToUseUnaryOnWrongType(op.clone().token_data, ty),
                            line: op.line, column: op.column,
                            })
                        }
                    },
                    _ => unreachable!(),
                }
            },
            ExpressionType::Assignment { ref target, ref op,  ref value } => {
                if let ExpressionType::Identifier(ref str) = target.expression_type {
                    let val = self.evaluate_expression(value, env)?;
                    let var_v = self.evaluate_expression(target, env)?;
                    let mut borrow = env.borrow_mut();
                    match op.token_data {
                        TokenData::Equal => {
                            let v = borrow.set(str, &val, true);
                            self.comply_def_err(v, value.line, value.column)?;
                        },
                        TokenData::AddAssign => {
                            let v;
                            match (var_v, val) {
                                (Value::Int(var), Value::Int(targ)) =>
                                    v = borrow.set(str, &Value::Int(var + targ), true),
                                (Value::Str(var), Value::Str(targ)) =>
                                    v = borrow.set(str, &Value::Str(Rc::new(format!("{}{}", var, targ))), true),
                                (Value::Float(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float(var + targ), true),
                                (Value::Int(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Int(var + targ as i64), true),
                                (var, _targ) => return Err(RuntimeError {
                                    error_type: RuntimeErrorType::CannotOperateOnType("+=".to_string(), format!("{} (value: {:?})", str, var)),
                                    line: op.line,
                                    column: op.column,
                                })
                            }
                            self.comply_def_err(v, value.line, value.column)?;
                        }
                        TokenData::SubAssign => {
                            let v;
                            match (var_v, val) {
                                (Value::Int(var), Value::Int(targ)) =>
                                    v = borrow.set(str, &Value::Int(var - targ), true),
                                (Value::Float(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float(var - targ), true),
                                (Value::Int(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float((var as f64) - targ), true),
                                (var, _targ) => return Err(RuntimeError {
                                    error_type: RuntimeErrorType::CannotOperateOnType("-=".to_string(), format!("{} (value: {:?})", str, var)),
                                    line: op.line,
                                    column: op.column,
                                })
                            }
                            self.comply_def_err(v, value.line, value.column)?;
                        }
                        TokenData::DivAssign => {
                            let v;
                            match (var_v, val) {
                                (Value::Int(var), Value::Int(targ)) =>
                                    v = borrow.set(str, &Value::Int(var / targ), true),
                                (Value::Float(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float(var / targ), true),
                                (Value::Int(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float((var as f64) / targ), true),
                                (var, _targ) => return Err(RuntimeError {
                                    error_type: RuntimeErrorType::CannotOperateOnType("/=".to_string(), format!("{}(value: {:?})", str, var)),
                                    line: op.line,
                                    column: op.column,
                                })
                            }
                            self.comply_def_err(v, value.line, value.column)?;
                        },
                        TokenData::MulAssign => {
                            let v;
                            match (var_v, val) {
                                (Value::Int(var), Value::Int(targ)) =>
                                    v = borrow.set(str, &Value::Int(var * targ), true),
                                (Value::Float(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float(var * targ), true),
                                (Value::Int(var), Value::Float(targ)) =>
                                    v = borrow.set(str, &Value::Float((var as f64) * targ), true),
                                (Value::Str(var), Value::Int(targ)) => {
                                    let mut full = String::new();
                                    for _ in 0..targ {
                                        full = format!("{}{}", full, var);
                                    }
                                    v = borrow.set(str, &Value::Str(Rc::new(full)), true);
                                }
                                (var, _targ) => return Err(RuntimeError {
                                    error_type: RuntimeErrorType::CannotOperateOnType("*=".to_string(), format!("{}(value: {:?})", str, var)),
                                    line: op.line,
                                    column: op.column,
                                })
                            }
                            self.comply_def_err(v, value.line, value.column)?;
                        }
                        _ => {}
                    }
                    Ok(Value::Null)
                } else {
                    todo!("Prop. access for assignments")
                }
            },
            ExpressionType::Integer(i) => Ok(Value::Int(i)),
            ExpressionType::Float(f) => Ok(Value::Float(f)),
            ExpressionType::Boolean(b) => Ok(Value::Bool(b)),
            ExpressionType::String(ref s) => Ok(Value::Str(Rc::new(s.clone()))),
            ExpressionType::Array(ref vec) => {
                let mut values = Vec::new();
                for v in vec {
                    let val = self.evaluate_expression(v, env)?;
                    let ty = val.to_type();
                    values.push((val, ty))
               };
                Ok(Value::Array(Rc::new(RefCell::new(values))))
            }
            ExpressionType::ArrayAccess { ref array, ref num } => {
                let arr_v = self.evaluate_expression(&*array, env)?;
                let num_v = self.evaluate_expression(&*num, env)?;
                if let Value::Array(vec) = arr_v {
                    if let Value::UInt(v) = num_v {
                        let borrow = vec.borrow();
                        if let Some((res, _ty)) = borrow.get(v as usize) {
                            Ok(res.clone())
                        } else {
                            Err(RuntimeError {
                                error_type: RuntimeErrorType::ArrIndexOutOfBounds(v as i64, vec.borrow().len()),
                                line: num.line,
                                column: num.column,
                            })
                        }
                    } else if let Value::Int(v) = num_v {
                        if v < 0 {
                            Err(RuntimeError {
                                error_type: RuntimeErrorType::ArrIndexOutOfBounds(v, vec.borrow().len()),
                                line: num.line,
                                column: num.column,
                            })
                        } else {
                            if let Some((res, _ty)) = vec.borrow().get(v as usize) {
                                Ok(res.clone())
                            } else {
                                Err(RuntimeError {
                                    error_type: RuntimeErrorType::ArrIndexOutOfBounds(v, vec.borrow().len()),
                                    line: num.line,
                                    column: num.column,
                                })
                            }
                        }
                    } else {
                        Err(RuntimeError {
                            error_type: RuntimeErrorType::AttemptToIndexArrWithNonInteger(num_v),
                            line: num.line,
                            column: num.column,
                        })
                    }
                } else {
                    Err(RuntimeError {
                        error_type: RuntimeErrorType::FailedEvaluatingExpression(expression.clone()),
                        line: array.line,
                        column: array.column
                    })
                }
            },
            ExpressionType::Identifier(ref name) => {
                if let Ok(var) = env.borrow().get(&name) {
                    Ok(var.value)
                } else {
                    Err(RuntimeError {
                        error_type: RuntimeErrorType::VariableNotFound(name.clone()),
                        line: expression.line,
                        column: expression.column,
                    })
                }
            }
            ExpressionType::PropertyAccess {ref object, ref property} => {
                let left = self.evaluate_expression(&object, env)?;
                match left {
                    Value::Object{name: _, ref fields} => {
                        let borrow = fields.borrow();
                        if let Some(v) = borrow.get(property) {
                            Ok(v.to_owned())
                        } else {
                            Err(RuntimeError {
                                error_type: RuntimeErrorType::PropertyNotFound(),
                                line: object.line,
                                column: object.column,
                            })
                        }
                    },
                    other => {
                        let ty = other.to_type();
                        if let Some(vtable) = self.vtables.get(&ty) {
                            if let Some(value) = vtable.get(property) {
                                return Ok(Value::BoundMethod {
                                    this: Box::new(other),
                                    func: Box::new(value.clone()),
                                })
                            }
                        }
                        Err(RuntimeError {
                            error_type: RuntimeErrorType::PropertyNotFound(),
                            line: object.line,
                            column: object.column,
                        })
                    }
                }
            },
            ExpressionType::Call {ref callee, ref args} => {
                let callee_v = self.evaluate_expression(callee, env)?;
                let mut arg_v = Vec::new();
                for arg in args {
                    let v = self.evaluate_expression(arg, env)?;
                    arg_v.push(v);
                }
                match callee_v {
                    Value::BoreFunction(func) => {
                        let v = self.run_function(func, arg_v, env);
                        v
                    },
                    Value::RustFunction { func, .. } => {
                        Ok(func(arg_v))
                    },
                    Value::BoundMethod { this, func } => {
                        arg_v.insert(0, *this);
                        match *func {
                            Value::RustFunction { func, .. } => {
                                 Ok(func(arg_v))
                            }
                            Value::BoreFunction(func) => {
                                let v = self.run_function(func, arg_v, env);
                                v
                            }
                            _ => unreachable!()
                        }
                    },
                    _ => Err(RuntimeError {
                        error_type: RuntimeErrorType::FailedEvaluatingExpression(expression.clone()),
                        line: callee.line,
                        column: callee.column,
                    })
                }
            },
            // _ => Err(RuntimeError{
            //     error_type: RuntimeErrorType::FailedEvaluatingExpression(expression.clone()),
            //     line: expression.line,
            //     column: expression.column,
            // }),
        }
    }

    pub fn run_function(&mut self, fn_stmt: Rc<Statement>, args: Vec<Value>, prev_env: &Env) -> Result<Value, RuntimeError> {
        let fn_env = Environment::new(Some(prev_env.clone()));
        let fn_env_rc = Rc::new(RefCell::new(fn_env));
        match fn_stmt.statement_type {
            StatementType::FunctionDeclaration {ref name, ref params, ref body, ref rtn} => {
                if params.iter().count() < args.iter().count() {
                    return Err(RuntimeError {
                        error_type: RuntimeErrorType::FunctionShippedWithWrongAmountOfArgs(name.clone(), params.iter().count(), args.iter().count()),
                        line: fn_stmt.line,
                        column: fn_stmt.column,
                    })
                }
                for i in 0..args.iter().count() {
                    let (name, ty, expr) = params.get(i).unwrap();
                    if let Some(arg) = args.get(i) {
                        let mut borrow = fn_env_rc.borrow_mut();
                        let v = borrow.define(name, Variable {is_mutable: true, ty: ty.clone(), value: arg.clone()});
                        self.comply_def_err(v, fn_stmt.line, fn_stmt.column)?;
                    } else {
                        if let Some(default) = expr {
                            let val = self.evaluate_expression(default, &fn_env_rc)?;
                            let mut borrow = fn_env_rc.borrow_mut();
                            let v =
                                borrow.define(name, Variable {is_mutable: true, ty: ty.clone(), value: val.clone()});
                            self.comply_def_err(v, fn_stmt.line, fn_stmt.column)?;
                        } else {
                            return Err(RuntimeError {
                                error_type: RuntimeErrorType::FunctionShippedWithWrongAmountOfArgs(name.clone(), params.iter().count(), args.iter().count()),
                                line: fn_stmt.line,
                                column: fn_stmt.column,
                            })
                        }
                    }
                }

                for stmt in body {
                    let res = self.run_statement(stmt, &fn_env_rc)?;
                    if let Some(ret) = res {
                        if !rtn.accepts(&ret) {
                            return Err(RuntimeError {
                                error_type: RuntimeErrorType::FnReturnsWrongType(name.clone(), rtn.clone(), ret.to_type()),
                                line: stmt.line,
                                column: stmt.column,
                            })
                        }
                        return Ok(ret)
                    }
                };
                Ok(Value::Null) // no return, null
            }
            _ => Err(RuntimeError{
                error_type: RuntimeErrorType::ExpectedDiff("function".to_string()),
                line: fn_stmt.line,
                column: fn_stmt.column,
            })
        }
    }

    pub fn run_body(&mut self, block: &Statement, prev_env: &Env) -> Result<Option<Value>, RuntimeError> {
        let bd_env = Environment::new(Some(prev_env.clone()));
        let rc = Rc::new(RefCell::new(bd_env));
        match block.statement_type {
            StatementType::If { ref condition, ref then_branch, ref else_branch} => {
                let condition_v = self.evaluate_expression(condition, &rc)?;
                match condition_v {
                    Value::Bool(true) => {
                        let v = self.iter_body(then_branch, &rc)?;
                        if let Some(v) = v {
                            return Ok(Some(v))
                        }
                    }
                    Value::Bool(false) => {
                        if let Some(else_branch) = else_branch {
                            let v = self.iter_body(else_branch, &rc)?;
                            if let Some(v) = v {
                                return Ok(Some(v))
                            }
                        }
                    }
                    _ => return Err(RuntimeError {
                        error_type: RuntimeErrorType::ExpectedDiff("boolean".to_string()),
                        line: condition.line,
                        column: condition.column,
                    })
                }
            },
            StatementType::WhileLoop { ref condition, ref body } => {
                loop {
                    let v = self.evaluate_expression(condition, &rc)?;
                    match v {
                        Value::Bool(true) => {
                            let v = self.iter_body(body, &rc)?;
                            if let Some(v) = v {
                                return Ok(Some(v))
                            }
                        },
                        Value::Bool(false) => return Ok(None),
                        _ => return Err(RuntimeError {
                            error_type: RuntimeErrorType::ExpectedDiff("boolean".to_string()),
                            line: condition.line,
                            column: condition.column,
                        })
                    }
                }
            }
            StatementType::ForLoop {ref var_decl, ref start, ref end, ref body} => {
                if let StatementType::VariableDeclaration { ref mutable, ref name, ref ty, ref value } = var_decl.statement_type {
                    if let Some(val) = value { // set val
                        let val_v = self.evaluate_expression(&val, &rc)?;
                        let var = Variable {
                            is_mutable: *mutable,
                            ty: ty.clone(),
                            value: val_v,
                        };
                        let mut borrow = rc.borrow_mut();
                        let v = borrow.define(&name, var);
                        self.comply_def_err(v, val.line, val.column)?;
                    } else { // not set
                        let val_v = self.evaluate_expression(start, &rc)?;
                        let var = Variable {
                            is_mutable: *mutable,
                            ty: ty.clone(),
                            value: val_v,
                        };
                        let mut borrow = rc.borrow_mut();
                        let v = borrow.define(&name, var);
                        self.comply_def_err(v, var_decl.line, var_decl.column)?;
                    }
                    let start_v = self.evaluate_expression(start, &rc)?;
                    let end_v = self.evaluate_expression(end, &rc)?;
                    match (start_v, end_v) {
                        (Value::Int(_start_v), Value::Int(end_v)) => {
                            loop {
                                let v = self.iter_body(body, &rc)?;
                                if let Some(v) = v {
                                    return Ok(Some(v))
                                }
                                let mut borrow = rc.borrow_mut();
                                let var = borrow.get(&name);
                                if var.is_err() {
                                    return Err(RuntimeError {
                                        error_type: var.unwrap_err(),
                                        line: start.line,
                                        column: start.column,
                                    })
                                }
                                let var = var.unwrap();
                                match var.value {
                                    Value::Int(val) => {
                                        let new = Value::Int(val + 1);
                                        let v = borrow.set(&name, &new, false);
                                        self.comply_def_err(v, start.line, start.column)?;
                                        if val + 1 == end_v {
                                            break
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {todo!("looping through strings, arrays etc")}
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }
    pub fn iter_body(&mut self, body: &Vec<Statement>, env: &Rc<RefCell<Environment>>) -> Result<Option<Value>, RuntimeError> {
        for stmt in body.iter() {
            let r = self.run_statement(stmt, env)?;
            if let Some(r) = r {
                return Ok(Some(r))
            }
        }
        Ok(None)
    }
    pub fn comply_def_err(&self, err: Result<&mut Environment, RuntimeErrorType>, line: usize, column: usize) -> Result<(), RuntimeError> {
        if let Err(e) = err {
            return Err(RuntimeError {
                error_type: e,
                line,
                column,
            })
        }
        Ok(())
    }
    pub fn compare_values<F>(&self, left: Value, right: Value, op: F, line: usize, column: usize) -> Result<Value, RuntimeError>
    where
        F: Fn(f64, f64) -> bool,
    {
        match (left, right) {
            (Value::Int(l), Value::Int(r)) =>
                Ok(Value::Bool(op(l as f64, r as f64))),
            (Value::Float(l), Value::Float(r)) =>
                Ok(Value::Bool(op(l, r))),
            (l, r) => Err(RuntimeError {
                error_type: RuntimeErrorType::Incompatible(l, r),
                line, column
            })
        }
    }
}