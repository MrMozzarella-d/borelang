use std::fmt::{Display, Formatter};
use crate::{Type, Value};
use crate::syntax::node::Expression;
use crate::syntax::token::TokenData;

#[derive(Debug)]
pub enum SyntaxErrorType {
    Expected(TokenData, TokenData),
    ExpectedLiteral(String, String),
    ExpectedAtomic(TokenData),
    Unexpected(TokenData),
    NullVariableWithoutType(String),
    TypeInferenceFailed(),
    InvalidAssignTarget,
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
            SyntaxErrorType::InvalidAssignTarget => {
                write!(f, "\x1b[31mSyntax Error\x1b[0m: Invalid assignment target at {}:{}", self.line, self.column)
            },
        }
    }
}
#[derive(Debug)]
pub enum RuntimeErrorType {
    VariableNotFound(String),
    VariableAlreadySet(String),
    TypeMismatch(Type, Type),
    //UnknownType(String),
    TypeInferenceFailed(),
    FailedEvaluatingExpression(Expression),
    CannotOperateOnType(String, String),
    Incompatible(Value, Value),
    CantAssignVariableToNull(),
    PropertyAccessOnTypeNotMap(),
    PropertyNotFound(),
    ExpectedDiff(String),
    Other(String),
    FunctionShippedWithWrongAmountOfArgs(String, usize, usize),
    UnexpectedToken(TokenData),
    AttemptToChangeConstantVar(String),
    FnReturnsWrongType(String, Type, Type),
    AttemptToUseUnaryOnWrongType(TokenData, Type),
    ModuleAlreadyImported(String),
    ModuleNotFound(String),
    ImportedModuleDoesntHaveInitFunction(String),
    ArrIndexOutOfBounds(i64, usize),
    AttemptToIndexArrWithNonInteger(Value)
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
            RuntimeErrorType::FailedEvaluatingExpression(ref e) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Failed evaluating expression {:?} ({}:{})", e.expression_type, self.line, self.column)
            },
            RuntimeErrorType::CannotOperateOnType(ref op, ref name_1) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Cannot operate with '{}' on {:?} ({}:{})", op, name_1, self.line, self.column)
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
            RuntimeErrorType::UnexpectedToken(ref tkn) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Unexpected token '{:?}' ({}:{})", tkn, self.line, self.column)
            }
            RuntimeErrorType::AttemptToChangeConstantVar(ref var) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Attempt to change constant '{}' ({}:{})", var, self.line, self.column)
            }
            RuntimeErrorType::TypeMismatch(ref expected, ref found) => {
                if expected != &Type::Any {
                    write!(f, "\x1b[31mRuntime Error\x1b[0m: Expected type '{:?}' but found '{:?}' ({}:{})", expected, found, self.line, self.column)
                } else {
                    write!(f, "\x1b[31mRuntime Error\x1b[0m: Expected any but somehow found '{:?}' ({}:{})", found, self.line, self.column)
                }
            }
            RuntimeErrorType::FnReturnsWrongType(ref name, ref ty, ref found) => {
                if ty == &Type::Any {
                    write!(f, "\x1b[31mRuntime Error\x1b[0m: Function '{}' returns wrong type, expected any but somehow found '{:?}' ({}:{})", name, found, self.line, self.column)
                } else {
                    write!(f, "\x1b[31mRuntime Error\x1b[0m: Function '{}' returns wrong type, expected '{:?}' but found '{:?}' ({}:{})", name, ty, found, self.line, self.column)
                }
            }
            RuntimeErrorType::AttemptToUseUnaryOnWrongType(ref op, ref ty) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Attempt to use unary '{:?}' on type '{:?}' ({}:{})", op, ty, self.line, self.column)
            }
            RuntimeErrorType::ModuleAlreadyImported(ref name) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Module '{}' already imported ({}:{})", name, self.line, self.column)
            }
            RuntimeErrorType::ModuleNotFound(ref name) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Module '{}' not found ({}:{})", name, self.line, self.column)
            }
            RuntimeErrorType::ImportedModuleDoesntHaveInitFunction(ref name) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Couldn't find an init function for module '{}' ({}:{})", name, self.line, self.column)
            }
            RuntimeErrorType::ArrIndexOutOfBounds(ref i, ref size) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Array index '{}' out of bounds (array has size of {}) ({}:{})", i, size, self.line, self.column)
            }
            RuntimeErrorType::AttemptToIndexArrWithNonInteger(ref nonint) => {
                write!(f, "\x1b[31mRuntime Error\x1b[0m: Attempt to index array with non-integer value '{}' ({}:{})", nonint, self.line, self.column)
            }
        }
    }
}