use crate::token::{Token};

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    //Object(Vec<(&'a str, Expression<'a>)>), <- i dont remember why i need this but i may in the future

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
        callee: Box<Expression>, // could also be prop access etc.
        args: Vec<Expression>,
    },
}
#[derive(Debug, Clone)]
pub struct Expression {
    pub expression_type: ExpressionType,
    pub line: usize,
    pub column: usize,
}
#[derive(Debug, Clone)]
pub enum StatementType {
    // Assignment { ; not needed, it gets put into a binaryOp
    //     target: Expression<'a>, // MAY be needed in the future
    //     value: Expression<'a>,
    // },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    VariableDeclaration {
        mutable: bool,
        name: String,
        ty: Option<Type>,
        value: Option<Expression>,
    },
    ForLoop {
        var_decl: Box<Statement>,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
    },
    Import {
        mods: Vec<String>,
    },
    Return(Option<Expression>),
    Expression(Expression),
}
#[derive(Debug, Clone)]
pub struct Statement {
    pub(crate) statement_type: StatementType,
    pub(crate) line: usize,
    pub(crate) column: usize,
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Type {
    Str,
    Int,
    Bool,
    Float,
    Null,
}
pub fn get_type(name: &String) -> Option<Type> {
    match name.as_str() {
        "str"  => Some(Type::Str),
        "int"  => Some(Type::Int),
        "bool" => Some(Type::Bool),
        "float" => Some(Type::Float),
        "null" => Some(Type::Null),
        _      => None,
    }
}

pub fn get_expr_type(expression: &Expression) -> Option<Type> {
    match expression.expression_type {
        ExpressionType::String(_) => Some(Type::Str),
        ExpressionType::Integer(_) => Some(Type::Int),
        ExpressionType::Float(_) => Some(Type::Float),
        ExpressionType::Boolean(_) => Some(Type::Bool),
        ExpressionType::BinaryOp { ref left, op: _, ref right } => {
            let left_ty = get_expr_type(left)?;
            let right_ty = get_expr_type(right)?;
            if left_ty == right_ty {
                Some(left_ty)
            } else {
                None
            }
        }
        _ => None,
    }
}