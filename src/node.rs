use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    StackReference(Token),
    NumberLiteral(f64),
    StringLiteral(String),
    ObjectLiteral(Vec<(String, Expression)>),

    BinaryOp {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },
    PropertyAccess {
        object: Box<Expression>,
        property: String,
    },
    Call {
        callee: String,
        args: Vec<Expression>,
    }
}
#[derive(Debug)]
pub enum Statement {
    Assignment {
        target: Expression,
        value: Expression,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Alias {
        slot: Expression,
        value: String,
    },
    ProcedureDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    Expression(Expression),
}

impl Statement {
    pub fn new_procedure(name: String, params: Vec<String>, body: Vec<Statement>) -> Self {
        Statement::ProcedureDeclaration { name, params, body }
    }
    pub fn new_call(callee: String, args: Vec<Expression>) -> Self {
        Statement::Expression(Expression::Call { callee, args })
    }
}

impl Expression {
    pub fn new_identifier(value: String) -> Self {
        Expression::Identifier(value)
    }
    pub fn new_number(value: f64) -> Self {
        Expression::NumberLiteral(value)
    }
    pub fn new_string_literal(value: String) -> Self {
        Expression::StringLiteral(value)
    }
}