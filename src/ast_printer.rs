use crate::node::{Expression, Statement};
use crate::token::TokenType;

pub struct ASTPrinter;

impl ASTPrinter {
    pub fn print_program(statements: &[Statement]) {
        for (i, stmt) in statements.iter().enumerate() {
            println!("[Statement {}]", i);
            Self::print_statement(stmt, 1);
        }
    }

    fn print_statement(stmt: &Statement, depth: usize) {
        let indent = "  ".repeat(depth);
        match stmt {
            Statement::ProcedureDeclaration { name, params, body } => {
                println!("{indent}ProcedureDeclaration:");
                println!("{indent}  Name: {name}");
                println!("{indent}  Params: {:?}", params);
                println!("{indent}  Body:");
                if body.is_empty() {
                    println!("{indent}    (Empty)");
                } else {
                    for (i, child_stmt) in body.iter().enumerate() {
                        println!("{indent}    [Inner Statement {}]", i);
                        Self::print_statement(child_stmt, depth + 3);
                    }
                }
            }
            Statement::Assignment { target, value } => {
                println!("{indent}Assignment:");
                println!("{indent}  Target:");
                Self::print_expression(target, depth + 2);
                println!("{indent}  Value:");
                Self::print_expression(value, depth + 2);
            }
            Statement::If { condition, then_branch, else_branch } => {
                println!("{indent}IfStatement:");
                println!("{indent}  Condition:");
                Self::print_expression(condition, depth + 2);
                println!("{indent}  Then:");
                for child_stmt in then_branch {
                    Self::print_statement(child_stmt, depth + 2);
                }
                if let Some(else_stmts) = else_branch {
                    println!("{indent}  Else:");
                    for child_stmt in else_stmts {
                        Self::print_statement(child_stmt, depth + 2);
                    }
                }
            }
            Statement::Alias { slot, value } => {
                println!("{indent}AliasDeclaration:");
                println!("{indent}  Slot:");
                Self::print_expression(slot, depth + 2);
                println!("{indent}  Value: {value}");
            }
            Statement::Expression(expr) => {
                println!("{indent}ExpressionStatement:");
                Self::print_expression(expr, depth + 1);
            }
        }
    }


    fn print_expression(expr: &Expression, depth: usize) {
        let indent = "  ".repeat(depth);
        match expr {
            Expression::Identifier(val) => {
                println!("{indent}Identifier: {val}");
            }
            Expression::NumberLiteral(val) => {
                println!("{indent}NumberLiteral: {val}");
            }
            Expression::StringLiteral(val) => {
                println!("{indent}StringLiteral: \"{val}\"");
            }
            Expression::StackReference(token) => {
                let ref_type = match token.token_type {
                    TokenType::StackAliasReference => "Alias",
                    TokenType::StackPointReference => "Point",
                    _ => "Unknown",
                };
                println!("{indent}StackReference ({ref_type}): {value}", value = token.value);
            }
            Expression::ObjectLiteral(fields) => {
                println!("{indent}ObjectLiteral:");
                for (key, val) in fields {
                    println!("{indent}  Field: {key}");
                    Self::print_expression(val, depth + 2);
                }
            }
            Expression::BinaryOp { left, op, right } => {
                println!("{indent}BinaryOp '{value}'", value = op.value);
                println!("{indent}  Left:");
                Self::print_expression(left, depth + 2);
                println!("{indent}  Right:");
                Self::print_expression(right, depth + 2);
            }
            Expression::PropertyAccess { object, property } => {
                println!("{indent}PropertyAccess:");
                println!("{indent}  Object:");
                Self::print_expression(object, depth + 2);
                println!("{indent}  Property: {property}");
            }
            Expression::Call { callee, args } => {
                println!("{indent}Call (Callee: {callee}):");
                if args.is_empty() {
                    println!("{indent}  Args: None");
                } else {
                    println!("{indent}  Args:");
                    for arg in args {
                        Self::print_expression(arg, depth + 2);
                    }
                }
            }
        }
    }
}
