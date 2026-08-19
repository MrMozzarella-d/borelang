use crate::token::Token;

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
