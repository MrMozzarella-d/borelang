use std::fmt::{Display, Formatter};
use crate::interpreter::{Value};
use crate::token::TokenData;

#[derive(Debug)]
pub enum SyntaxErrorType {
    Expected(TokenData, TokenData),
    ExpectedLiteral(String, String),
    ExpectedAtomic(TokenData),
    Unexpected(TokenData),
    NullVariableWithoutType(String),
    TypeInferenceFailed(),
}
#[derive(Debug)]
pub struct SyntaxError {
    pub error_type: SyntaxErrorType,
    pub line: usize,
    pub column: usize,
}
impl Display for SyntaxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ty = &self.error_type;
        match ty {
            SyntaxErrorType::Expected(expected, got) => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Expected type {:?} but found {:?} at {}:{}", expected, got, self.line, self.column)
            }
            SyntaxErrorType::Unexpected(ty) => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Unexpected {:?} at {}:{}.", ty, self.line, self.column)
            }
            SyntaxErrorType::ExpectedAtomic(found) => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Expected atomic but found {:?} at {}:{}", found, self.line, self.column)
            }
            SyntaxErrorType::ExpectedLiteral(expected, got) => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Expected '{}' but found '{}' at {}:{}", expected, got, self.line, self.column)
            }
            SyntaxErrorType::NullVariableWithoutType(name) => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Either type annotation or value needed, got neither for variable '{}' at {}:{}", name, self.line, self.column)
            }
            SyntaxErrorType::TypeInferenceFailed() => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Type Inference Failed at {}:{}", self.line, self.column)
            }
            // SyntaxErrorType::TypeError(expected, found) => {
            //     write!(f, "Syntax Error: Expected type {:?}, found {:?} at {}:{}", expected, found, self.line, self.column)
            // }
        }
    }
}
#[derive(Debug)]
pub enum RuntimeErrorType {
    VariableNotFound(String),
    VariableAlreadySet(String),
    //TypeError(String),
    //UnknownType(String),
    TypeInferenceFailed(),
    FailedEvaluatingExpression(),
    CannotOperateOnType(String, String),
    Incompatible(Value, Value),
    CantAssignVariableToNull(),
    PropertyAccessOnTypeNotMap(),
    PropertyNotFound(),
    ExpectedDiff(String),
    Other(String),
    FunctionShippedWithWrongAmountOfArgs(String, usize, usize),
}
#[derive(Debug)]
pub struct RuntimeError {
    pub error_type: RuntimeErrorType,
    pub line: usize,
    pub column: usize,
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            RuntimeErrorType::VariableAlreadySet(ref var) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Variable '{}' already exists in current scope ({}:{})", var, self.line, self.column)
            },
            RuntimeErrorType::VariableNotFound(ref var) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Variable '{}' not found in current scope ({}:{})", var, self.line, self.column)
            },
            RuntimeErrorType::TypeInferenceFailed() => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Type inference failed ({}:{})", self.line, self.column)
            },
            RuntimeErrorType::FailedEvaluatingExpression() => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Failed evaluating expression ({}:{})", self.line, self.column)
            },
            RuntimeErrorType::CannotOperateOnType(ref op, ref str) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Cannot operate with '{}' on {:?} ({}:{})", op, str, self.line, self.column)
            },
            RuntimeErrorType::Incompatible(ref one, ref two) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Incompatible '{:?}' and '{:?}' ({}:{})", one, two, self.line, self.column)
            },
            RuntimeErrorType::CantAssignVariableToNull() => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Cannot assign a variable to type null ({}:{})", self.line, self.column)
            },
            RuntimeErrorType::PropertyAccessOnTypeNotMap() => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Attempt to access a property on a type that is not a map ({}:{})", self.line, self.column)
            },
            RuntimeErrorType::PropertyNotFound() => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Property not found ({}:{})", self.line, self.column)
            },
            RuntimeErrorType::ExpectedDiff(ref ty) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Expected '{}' ({}:{})", ty, self.line, self.column)
            },
            RuntimeErrorType::Other(ref s) => {
                write!(f, "{} ({}:{})", s, self.line, self.column)
            }
            RuntimeErrorType::FunctionShippedWithWrongAmountOfArgs(ref s, ref expected, ref got) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Function '{}' expected {} arguments, but got {} ({}:{})", s, expected, got, self.line, self.column)
            }
        }
    }
}