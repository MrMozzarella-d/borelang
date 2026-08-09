use crate::token::Token;

#[derive(Debug)]
pub enum Expression<'a> {
    Identifier(&'a str),
    Integer(i64),
    Float(f64),
    StringLiteral(String),
    ObjectLiteral(Vec<(&'a str, Expression<'a>)>),

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
    FunctionDeclaration {
        name: &'a str,
        params: Vec<&'a str>,
        body: Vec<Statement<'a>>,
    },
    Return(Option<Expression<'a>>),
    Expression(Expression<'a>),
}