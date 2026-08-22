use phf::phf_map;
use crate::runtime::interpreter::{Type};
use crate::syntax::token::{Token};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionType {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Expression>),
    BinaryOp {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    Unary {
        op: Token,
        right: Box<Expression>
    },
    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    Assignment {
        target: Box<Expression>,
        op: Token,
        value: Box<Expression>,
    },
    ArrayAccess {
        array: Box<Expression>,
        num: Box<Expression>,
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub line: usize,
    pub column: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub enum StatementType {
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<(String, Type, Option<Expression>)>,
        body: Vec<Statement>,
        rtn: Type,
    },
    VariableDeclaration {
        mutable: bool,
        name: String,
        ty: Type,
        value: Option<Expression>,
    },
    ForLoop {
        var_decl: Box<Statement>,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
    },
    WhileLoop {
        condition: Expression,
        body: Vec<Statement>
    },
    Import {
        mods: Vec<String>,
    },
    Return(Option<Expression>),
    Expression(Expression),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub(crate) statement_type: StatementType,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

pub static PRIMITIVES: phf::Map<&'static str, Type> = phf_map! {
    "uint" => Type::UInt,
    "int" => Type::Int,
    "str" => Type::Str,
    "bool" => Type::Bool,
    "float" => Type::Float,
    "null" => Type::Null,
    "any" => Type::Any,
};