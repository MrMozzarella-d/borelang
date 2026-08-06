use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str), // IDENTIFIERS ARE NOT REAL THEY ARE JUST PROPAGANDA MADE FROM BIG C SO THEY CAN SELL YOU MORE COURSES
    StackReference(Token<'a>),                    // DENNIS RITCHIE JUST WANTED TO HAVE A MONOPOLY ON LANGUAGES THATS WHY IDENTIFIERS EXIST
    NumberLiteral(f64),                           // just kidding haha :D
    StringLiteral(String),
    ObjectLiteral(Vec<(&'a str, Expression<'a>)>),

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
        callee: &'a str,
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
    Alias {
        slot: Expression<'a>,
        value: &'a str,
    },
    ProcedureDeclaration {
        name: &'a str,
        params: Vec<&'a str>,
        body: Vec<Statement<'a>>,
    },
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
}

impl<'a> Statement<'a> {
    pub fn new_procedure(name: &'a str, params: Vec<&'a str>, body: Vec<Statement<'a>>) -> Self {
        Statement::ProcedureDeclaration { name, params, body }
    }
    pub fn new_call(callee: &'a str, args: Vec<Expression<'a>>) -> Self {
        Statement::Expression(Expression::Call { callee, args })
    }
}

impl<'a> Expression<'a> {
    pub fn new_stack_reference(reference: Token<'a>) -> Self {
        Expression::StackReference(reference)
    }
    pub fn new_number(value: f64) -> Self {

        Expression::NumberLiteral(value)
    }
    pub fn new_string_literal(value: String) -> Self {
        Expression::StringLiteral(value)
    }
    pub fn new_binary_op(left: Box<Expression<'a>>, op: Token<'a>, right: Box<Expression<'a>>) -> Self {
        Expression::BinaryOp { left, op, right }
    }
}