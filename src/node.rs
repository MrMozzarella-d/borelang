use crate::token::{Token};

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
        property: &'a str,
    },
    Call {
        callee: Box<Expression<'a>>,
        args: Vec<Expression<'a>>, // 14/08/26: norrr im too lazzzzyyy...... im sooo gassssyyyyyy im farting all over the place eeee e oe ke kweaop w2424pou094ßu 39042uqß4nigrrdgngg
    },                            // also 14/08/26: i am not lazy anymore.                             pet me.
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
    ForLoop {
        var_decl: Box<Statement<'a>>,
        start: Expression<'a>,
        end: Expression<'a>,
        body: Vec<Statement<'a>>,
    },
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
}

#[derive(Debug)]
pub enum Type {
    Str,
    Int,
    Bool,
}
pub fn get_type(name: &str) -> Option<Type> {
    match name {
        "str"  => Some(Type::Str),
        "int"  => Some(Type::Int),
        "bool" => Some(Type::Bool),
        _      => None,
    }
}