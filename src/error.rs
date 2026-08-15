use std::fmt::{write, Display, Formatter};
use crate::token::TokenData;

#[derive(Debug)]
pub enum SyntaxErrorType<'a> {
    Expected(TokenData<'a>, TokenData<'a>),
    ExpectedLiteral(String, String),
    ExpectedAtomic(TokenData<'a>),
    Unexpected(TokenData<'a>),
    NullVariableWithoutType(String),
}
#[derive(Debug)]
pub struct SyntaxError<'a> {
    pub error_type: SyntaxErrorType<'a>,
    pub line: usize,
    pub column: usize,
}
impl Display for SyntaxError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ty = &self.error_type;
        match ty {
            SyntaxErrorType::Expected(expected, got) => {
                write!(f, "Syntax Error: Expected type {:?} but found {:?} at {}:{}", expected, got, self.line, self.column)
            }
            SyntaxErrorType::Unexpected(ty) => {
                write!(f, "Syntax Error: Unexpected {:?} at {}:{}.", ty, self.line, self.column)
            }
            SyntaxErrorType::ExpectedAtomic(found) => {
                write!(f, "Syntax Error: Expected atomic but found {:?} at {}:{}", found, self.line, self.column)
            }
            SyntaxErrorType::ExpectedLiteral(expected, got) => {
                write!(f, "Syntax Error: Expected '{}' but found '{}' at {}:{}", expected, got, self.line, self.column)
            }
            SyntaxErrorType::NullVariableWithoutType(name) => {
                write!(f, "Syntax Error: Either type annotation or value needed, got neither for variable '{}' at {}:{}", name, self.line, self.column)
            }
        }
    }
}

pub enum RuntimeErrorType {
    VariableNotFound(String),
    VariableAlreadySet(String),
}
pub struct RuntimeError {
    pub error_type: RuntimeErrorType,
    pub line: usize,
    pub column: usize,
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            RuntimeErrorType::VariableAlreadySet(ref var) => {
                write!(f, "Runtime Error: Variable '{}' already exists in current scope ({}:{})", var, self.line, self.column)
            },
            RuntimeErrorType::VariableNotFound(ref var) => {
                write!(f, "Runtime Error: Variable '{}' not found in current scope ({}:{})", var, self.line, self.column)
            }
        }
    }
}