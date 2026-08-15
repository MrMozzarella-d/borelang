use crate::token::{Token};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Integer(i64),
    Float(f64),
    String(&'a str),
    Boolean(bool),
    //Object(Vec<(&'a str, Expression<'a>)>), <- i dont remember why i need this but i may in the future

    BinaryOp {
        left: Box<Expression<'a>>,
        op: Token<'a>,
        right: Box<Expression<'a>>,
    },
    PropertyAccess {
        object: Box<Expression<'a>>,
        property: &'a str,
    },
    Call {
        callee: Box<Expression<'a>>, // could also be prop access etc.
        args: Vec<Expression<'a>>,
    },
}
#[derive(Debug, Clone)]
pub enum StatementType<'a> {
    // Assignment { ; not needed, it gets put into a binaryOp
    //     target: Expression<'a>, // MAY be needed in the future
    //     value: Expression<'a>,
    // },
    If {
        condition: Expression<'a>,
        then_branch: Vec<Statement<'a>>,
        else_branch: Option<Vec<Statement<'a>>>,
    },
    FunctionDeclaration {
        name: &'a str,
        params: Vec<&'a str>,
        body: Vec<Statement<'a>>,
    },
    VariableDeclaration {
        mutable: bool,
        name: &'a str,
        ty: Option<Type>,
        value: Option<Expression<'a>>,
    },
    ForLoop {
        var_decl: Box<Statement<'a>>,
        start: Expression<'a>,
        end: Expression<'a>,
        body: Vec<Statement<'a>>,
    },
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
}
#[derive(Debug, Clone)]
pub struct Statement<'a> {
    pub(crate) statement_type: StatementType<'a>,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Type {
    Str,
    Int,
    Bool,
    Float,
}
pub fn get_type(name: &str) -> Option<Type> {
    match name {
        "str"  => Some(Type::Str),
        "int"  => Some(Type::Int),
        "bool" => Some(Type::Bool),
        "float" => Some(Type::Float),
        _      => None,
    }
}

pub fn get_op_expr_type(expression: &Option<Expression>) -> Option<Type> {
    match expression {
        Some(Expression::String(_)) => Some(Type::Str),
        Some(Expression::Integer(_)) => Some(Type::Int),
        Some( Expression::Float(_)) => Some(Type::Float),
        Some(Expression::Boolean(_)) => Some(Type::Bool),
        _ => None,
    }
}