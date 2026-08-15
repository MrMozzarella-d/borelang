use std::collections::HashMap;
use crate::error::{RuntimeError, RuntimeErrorType};
use crate::node::{Statement, StatementType, Type};

#[derive(Clone)]
pub enum RuntimeValue {
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}
pub struct Variable {
    is_mutable: bool,
    ty: Type,
    value: RuntimeValue,
}
pub struct Interpreter<'a> {
    ast: Vec<Statement<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new(ast: Vec<Statement<'a>>) -> Self {
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
                if let Some(expr) = value.clone() { // w/ value
                    
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
}