use crate::token::{Token, TokenData};

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Integer(i64),
    Float(f64),
    String(&'a str),
    Boolean(bool),
    Object(Vec<(&'a str, Expression<'a>)>),

    BinaryOp {
        left: Box<Expression<'a>>,
        op: Token<'a>,
        right: Box<Expression<'a>>,
    },
    PropertyAccess {
        object: Box<Expression<'a>>,
        property: Box<Expression<'a>>,
    },
    Call {
        callee: &'a str, // todo: change this to a expression
        args: Vec<Expression<'a>>,
    }
}
#[derive(Debug)]
pub enum Statement<'a> {
    Assignment {
        target: Expression<'a>,
        value: Expression<'a>,
    },
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
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub enum Type {
    Str,
    Int,
}
pub fn get_type(name: &str) -> Option<Type> {
    match name {
        "str" => Some(Type::Str),
        "int" => Some(Type::Int),
        _ => None,
    }
}