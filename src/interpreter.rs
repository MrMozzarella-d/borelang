use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::builtins;
use crate::error::{RuntimeError, RuntimeErrorType};
use crate::node::{Statement, StatementType, ExpressionType, Expression};
use crate::token::TokenData;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(Rc<String>),
    Bool(bool),
    List(Rc<RefCell<Vec<Value>>>),
    Object {
        name: String,
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
    BoreFunction(Rc<Statement>),
    RustFunction(fn(args: Vec<Value>) -> Result<Value, RuntimeErrorType>),
    Null,
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub enum Type {
//     Int,
//     Float,
//     Str,
//     Bool,
//     Null,
//     Custom(String),
//     Unresolved(String),
// }
// pub fn get_expr_type(expression: &Expression) -> Option<Type> {
//     match expression.expression_type {
//         ExpressionType::String(_) => Some(Type::Str),
//         ExpressionType::Integer(_) => Some(Type::Int),
//         ExpressionType::Float(_) => Some(Type::Float),
//         ExpressionType::Boolean(_) => Some(Type::Bool),
//         ExpressionType::BinaryOp { ref left, op: _, ref right } => {
//             let left_ty = get_expr_type(left)?;
//             let right_ty = get_expr_type(right)?;
//             if left_ty == right_ty {
//                 Some(left_ty)
//             } else {
//                 None
//             }
//         }
//         _ => None,
//     }
// }
// #[derive(Clone, Debug)]
// pub enum TypeRule {
//     Explicit(Type),
//     Dynamic,
// }
#[derive(Clone, Debug)]
pub struct Variable {
    pub is_mutable: bool,
    pub value: Value,
}
pub struct Interpreter {
    ast: Vec<Statement>,
}
#[derive(Clone, Debug)]
pub struct Environment {
    values: HashMap<String, Variable>,
    enclosing: Option<Rc<RefCell<Environment>>>
}
impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }
    pub fn define(&mut self, name: String, var: Variable) -> Result<&mut Environment, RuntimeErrorType> {
        if self.values.contains_key(&name) {
            return Err(RuntimeErrorType::VariableAlreadySet(name));
        }
        self.values.insert(name, var);
        Ok(self)
    }
    pub fn get(&self, name: &String) -> Result<Variable, RuntimeErrorType> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        if let Some(ref enc) = self.enclosing {
            let borrow = enc.borrow();
            let v = borrow.get(name);
            return v;
        }
        Err(RuntimeErrorType::VariableNotFound(name.clone()))
    }
}
impl Interpreter {
    pub fn new(ast: Vec<Statement>) -> Self {
        Self {
            ast,
        }
    }
    pub fn run(&self) -> Result<(), RuntimeError> {
        let mut global_env = Environment::new(None);
        // built-ins module
        let builtins = builtins::Module::new();
        match builtins.obj {
            Value::Object { name: _, fields } => {
                for field in fields.borrow().iter() {
                    let var = Variable {
                        is_mutable: false,
                        value: field.1.clone(),
                    };
                    let v = global_env.define(field.0.clone(), var);
                    if v.is_err() {
                        return Err(RuntimeError {
                            error_type: v.unwrap_err(),
                            line: 1,
                            column: 1,
                        })
                    };
                }
            }
            _ => {} // this CANNOT happen
        }
        // pre-register of all functions
        for stmt in self.ast.iter() {
            if let StatementType::FunctionDeclaration { ref name, params: _, body: _ } = stmt.statement_type {
                let func_v = Value::BoreFunction(Rc::new(stmt.clone()));
                let var = Variable {
                    is_mutable: false,
                    value: func_v,
                };
                let v = global_env.define(name.clone(), var);
                if v.is_err() {
                    return Err(RuntimeError {
                        error_type: v.unwrap_err(),
                        line: stmt.line,
                        column: stmt.column, 
                    })
                }
            }
        }

        let rc = Rc::new(RefCell::new(global_env));
        for stmt in self.ast.iter() {
            if let Err(e) = self.run_statement(stmt, &rc) {
                return Err(e);
            }
        }
        Ok(())
    }
    pub fn run_statement(&self, stmt: &Statement, env: &Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
        let ty = &stmt.statement_type;
        match ty {
            StatementType::VariableDeclaration { mutable, name, value } => {
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
                    let var = Variable {
                        is_mutable: *mutable,
                        value: runtime,
                    };
                    let mut borrow = env.borrow_mut();
                    let v = borrow.define(name, var);
                    if v.is_err() {
                        return Err(RuntimeError {
                            error_type: v.unwrap_err(),
                            line: value.line,
                            column: value.column,
                        })
                    }
                } else { // none
                    let var = Variable {
                        is_mutable: *mutable,
                        value: Value::Null,
                    };
                    let mut borrow = env.borrow_mut();
                    let v = borrow.define(name, var);
                    if v.is_err() {
                        return Err(RuntimeError {
                            error_type: v.unwrap_err(),
                            line: stmt.line,
                            column: stmt.column,
                        })
                    }
                }
                Ok(())
            },
            StatementType::Expression(expr) => {
                let v = self.evaluate_expression(&expr, env);
                let result = v.map(|_| ());
                result
            },
            _ => { Ok(()) }
        }
    }
    
    pub fn evaluate_expression(&self, expression: &Expression, env: &Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
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
                    _ => todo!("Assignment operators (AddAssign, Assignment, etc.)")
                }
            },
            ExpressionType::Integer(i) => Ok(Value::Int(i)),
            ExpressionType::Boolean(b) => Ok(Value::Bool(b)),
            ExpressionType::String(ref s) => Ok(Value::Str(Rc::new(s.clone()))),
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
                    Value::Object{ref name, ref fields} => {
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
                    _ => Err(RuntimeError {
                        error_type: RuntimeErrorType::PropertyAccessOnTypeNotMap(),
                        line: object.line,
                        column: object.column,
                    })
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
                    Value::RustFunction(func) => {
                        func(arg_v)
                            .map_err(|ty| RuntimeError {
                                error_type: ty,
                                line: callee.line,
                                column: callee.column,
                            })
                    },
                    _ => Err(RuntimeError {
                        error_type: RuntimeErrorType::FailedEvaluatingExpression(),
                        line: callee.line,
                        column: callee.column,
                    })
                }
            },
            _ => Err(RuntimeError{
                error_type: RuntimeErrorType::FailedEvaluatingExpression(),
                line: expression.line,
                column: expression.column,
            }),
        }
    }

    pub fn run_function(&self, fn_stmt: Rc<Statement>, args: Vec<Value>, prev_env: &Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
        let fn_env = Environment::new(Some(prev_env.clone()));
        let fn_env_rc = Rc::new(RefCell::new(fn_env));
        match fn_stmt.statement_type {
            StatementType::FunctionDeclaration {ref name, ref params, ref body} => {
                for i in 0..params.iter().count() {
                    let param = params.get(i).unwrap();
                    if let Some(arg) = args.get(i) {
                        let mut borrow = fn_env_rc.borrow_mut();
                        let v = borrow.define(param.clone(), Variable {is_mutable: true, value: arg.clone()});
                        if v.is_err() {
                            return Err(RuntimeError {
                                error_type: v.unwrap_err(),
                                line: fn_stmt.line,
                                column: fn_stmt.column,
                            })
                        }
                    } else {
                        return Err(RuntimeError {
                            error_type: RuntimeErrorType::FunctionShippedWithWrongAmountOfArgs(name.clone(), params.iter().count(), args.iter().count()),
                            line: fn_stmt.line,
                            column: fn_stmt.column,
                        })
                    }
                }

                for stmt in body {
                    self.run_statement(stmt, &fn_env_rc)?;
                };
                Ok(Value::Null) // todo: replace this placeholder lazy fuck
            }
            _ => Err(RuntimeError{
                error_type: RuntimeErrorType::ExpectedDiff("function".to_string()),
                line: fn_stmt.line,
                column: fn_stmt.column,
            })
        }
    }
}