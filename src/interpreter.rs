use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::__private_api::Value;
use crate::error::{RuntimeError, RuntimeErrorType};
use crate::node::{get_expr_type, Statement, StatementType, Type, ExpressionType, Expression};
use crate::token::TokenData;

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Map(HashMap<String, RuntimeValue>),
    RustFunction(fn(Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeErrorType>),
    BoreFunction(Rc<Statement>),
    Null,
}
pub struct Variable {
    is_mutable: bool,
    ty: Type,
    value: RuntimeValue,
}
pub struct Interpreter {
    env: Environment,
    ast: Vec<Statement>,
}
// impl RuntimeValue {
//     pub fn from(value: &ExpressionType) -> Option<Self> {
//         match value {
//             ExpressionType::Boolean(b) => Some(RuntimeValue::Bool(*b)),
//             ExpressionType::Integer(i) => Some(RuntimeValue::Int(*i)),
//             ExpressionType::Float(f) => Some(RuntimeValue::Float(*f)),
//             ExpressionType::String(s) => Some(RuntimeValue::Str(s.to_string())),
//             _ => None,
//         }
//     }
// }
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, RuntimeValue>,
    enclosing: Option<Rc<RefCell<Environment>>>
}
impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }
    pub fn define(&mut self, name: String, value: RuntimeValue) -> Result<&mut Environment, RuntimeErrorType> {
        if self.values.contains_key(&name) {
            return Err(RuntimeErrorType::VariableAlreadySet(name));
        }
        self.values.insert(name, value);
        Ok(self)
    }
    pub fn get(&self, name: String) -> Result<RuntimeValue, RuntimeErrorType> {
        if let Some(value) = self.values.get(&name) {
            return Ok(value.clone());
        }
        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().borrow().get(name);
        }
        Err(RuntimeErrorType::VariableNotFound(name))
    }
}
impl Interpreter {
    pub fn new(ast: Vec<Statement>) -> Self {
        Self {
            ast,
        }
    }
    pub fn run(&self) -> Result<(), RuntimeError> {
        let mut global_scope = HashMap::<String, Variable>::new();
        for stmt in self.ast.iter() {
            if let Err(e) = self.run_statement(stmt, &mut global_scope) {
                return Err(e);
            }
        }
        Ok(())
    }
    pub fn run_statement(&self, stmt: &Statement, scope: &mut HashMap<String, Variable>) -> Result<(), RuntimeError> {
        let ty = &stmt.statement_type;
        match ty {
            StatementType::VariableDeclaration { mutable, name, ty, value } => {
                let name = name.to_string();
                if scope.contains_key(&name) {
                    return Err(RuntimeError {
                        error_type: RuntimeErrorType::VariableAlreadySet(name),
                        line: stmt.line,
                        column: stmt.column,
                    })
                }
                if let Some(value) = value.clone() { // w/ value
                    let runtime = self.evaluate_expression(&value, scope)?;
                    if let Some(ty) = ty { // explicit
                        if ty == &Type::Null {
                            return Err(RuntimeError {
                                error_type: RuntimeErrorType::CantAssignVariableToNull(),
                                line: value.line,
                                column: value.column,
                            })
                        }
                        let var = Variable {
                            is_mutable: *mutable,
                            ty: *ty,
                            value: runtime,
                        };
                        scope.insert(name, var);
                    } else { // none
                        let ty = get_expr_type(&value);
                        if let Some(ty) = ty {
                            let var = Variable {
                                is_mutable: *mutable,
                                ty,
                                value: runtime,
                            };
                            scope.insert(name, var);
                        } else {
                            let line = value.line;
                            let column = value.column;
                            return Err(RuntimeError{
                                error_type: RuntimeErrorType::TypeInferenceFailed(),
                                line,
                                column,
                            })
                        }
                    }
                } else { // w/o value
                    if let Some(ty) = ty.clone() {
                        let var = Variable {
                            is_mutable: *mutable,
                            ty,
                            value: RuntimeValue::Null,
                        };
                        scope.insert(name, var);
                    };
                }
                Ok(())
            }
            _ => { Ok(()) }
        }
    }
    
    pub fn evaluate_expression(&self, expression: &Expression, scope: &mut HashMap<String, Variable>) -> Result<RuntimeValue, RuntimeError> {
        match expression.expression_type {
            ExpressionType::BinaryOp{ref left, ref op, ref right} => {
                let left_v = self.evaluate_expression(left, scope)?;
                let right_v = self.evaluate_expression(right, scope)?;
                match op.token_data {
                    TokenData::Add => match (left_v, right_v) {
                        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(l + r)),
                        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l + r)),
                        (RuntimeValue::Str(l), RuntimeValue::Str(r)) => Ok(RuntimeValue::Str(l + &r)),
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::IncompatibleTypes(
                                get_expr_type(left).unwrap_or(Type::Null),
                                get_expr_type(right).unwrap_or(Type::Null),
                            ),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Sub => match (left_v, right_v) {
                        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(l - r)),
                        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l - r)),
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::IncompatibleTypes(
                                get_expr_type(left).unwrap_or(Type::Null),
                                get_expr_type(right).unwrap_or(Type::Null),
                            ),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Mul => match (left_v, right_v) {
                        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(l * r)),
                        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l * r)),
                        (RuntimeValue::Str(l), RuntimeValue::Int(r)) => {
                            let mut str = String::new();
                            for _ in 0..r {
                                str = format!("{}{}", str, l);
                            };
                            Ok(RuntimeValue::Str(str))
                        }
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::IncompatibleTypes(
                                get_expr_type(left).unwrap_or(Type::Null),
                                get_expr_type(right).unwrap_or(Type::Null),
                            ),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    TokenData::Div => match (left_v, right_v) {
                        (RuntimeValue::Int(l), RuntimeValue::Int(r)) => Ok(RuntimeValue::Int(l / r)),
                        (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l / r)),
                        (_l_val, _r_val) => Err(RuntimeError {
                            error_type: RuntimeErrorType::IncompatibleTypes(
                                get_expr_type(left).unwrap_or(Type::Null),
                                get_expr_type(right).unwrap_or(Type::Null),
                            ),
                            line: right.line,
                            column: right.column,
                        })
                    },
                    _ => todo!("Assignment operators (AddAssign, Assignment, etc.)")
                }
            },
            ExpressionType::Integer(i) => Ok(RuntimeValue::Int(i)),
            ExpressionType::Boolean(b) => Ok(RuntimeValue::Bool(b)),
            ExpressionType::String(ref s) => Ok(RuntimeValue::Str(s.to_string())),
            // ExpressionType::Identifier(name) => {
            //     if let Some(val) = scope.get(&name) {
            //         self.evaluate_expression(val)
            //     } else {
            //         Err(RuntimeError {
            //             error_type: RuntimeErrorType::VariableNotFound(name),
            //             line: expression.line,
            //             column: expression.column,
            //         })
            //     }
            // }
            ExpressionType::PropertyAccess {ref object, ref property} => {
                let left = self.evaluate_expression(&object, scope)?;
                match left {
                    RuntimeValue::Map(ref map) => {
                        if let Some(v) = map.get(property) {
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
                let callee_v = self.evaluate_expression(callee, scope)?;
                let mut arg_v = Vec::new();
                for arg in args {
                    let v = self.evaluate_expression(arg, scope)?;
                    arg_v.push(v);
                }
                match callee_v {
                    RuntimeValue::BoreFunction(func) => {
                        let v = self.run_function(func, arg_v);
                        v
                    },
                    RuntimeValue::RustFunction(func) => {
                        let v = func(arg_v);
                        if v.is_err() {
                            return Err(RuntimeError {
                                error_type: v.unwrap_err(),
                                line: callee.line,
                                column: callee.column,
                            });
                        };
                        v.map_err(|typ| RuntimeError { error_type: typ, line: callee.line, column: callee.column })
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
            })
        }
    }

    pub fn run_function(&self, fn_stmt: Rc<Statement>, args: Vec<RuntimeValue>) -> Result<RuntimeValue, RuntimeError> {
        match fn_stmt.statement_type {
            StatementType::FunctionDeclaration {ref name, ref params, ref body} => {
                for stmt in body {
                    self.run_statement(stmt, )
                }
            }
        }
    }
}