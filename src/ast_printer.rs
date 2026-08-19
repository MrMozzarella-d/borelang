use crate::node::{Statement, Expression, StatementType, ExpressionType};

pub struct AstTreePrinter {
    indent_level: usize,
}
impl AstTreePrinter {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }
    fn indent(&mut self) {
        self.indent_level += 1;
    }
    fn outdent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    fn p(&self, text: &str) -> String {
        format!("{}{}\n", "  ".repeat(self.indent_level), text)
    }

    pub fn print_program(&mut self, statements: &[Statement]) {
        for stmt in statements {
            print!("{}", self.print_statement(stmt));
        }
    }
    pub fn print_statement(&mut self, stmt: &Statement) -> String {
        match &stmt.statement_type {
            StatementType::VariableDeclaration { mutable, name, value } => {
                let mut r = self.p(&format!("VariableDeclaration (name: '{}', mut: {})", name, mutable));
                self.indent();
                if let Some(v) = value {
                    r.push_str(&self.p("Value:"));
                    self.indent();
                    r.push_str(&self.print_expression(v));
                    self.outdent();
                } else {
                    r.push_str(&self.p("No value assigned"));
                }
                self.outdent();
                r
            }
            StatementType::Expression(expr) => {
                let mut r = self.p("ExpressionStatement");
                self.indent();
                r.push_str(&self.print_expression(expr));
                self.outdent();
                r
            }
            StatementType::Return(value) => {
                let mut r = self.p("Return");
                if let Some(expr) = value {
                    self.indent();
                    r.push_str(&self.print_expression(expr));
                    self.outdent();
                }
                r
            }
            StatementType::Import { mods } => {
                let mut r = self.p("ImportStatement");
                self.indent();
                r.push_str(&self.p(&format!("Modules: {:?}", mods)));
                self.outdent();
                r
            }
            StatementType::If { condition, then_branch, else_branch } => {
                let mut r = self.p("IfStatement");
                self.indent();
                r.push_str(&self.p("Condition:"));
                self.indent();
                r.push_str(&self.print_expression(condition));
                self.outdent();

                r.push_str(&self.p("Then:"));
                self.indent();
                for inner_stmt in then_branch {
                    r.push_str(&self.print_statement(inner_stmt));
                }
                self.outdent();

                if let Some(else_stmts) = else_branch {
                    r.push_str(&self.p("Else:"));
                    self.indent();
                    for inner_stmt in else_stmts {
                        r.push_str(&self.print_statement(inner_stmt));
                    }
                    self.outdent();
                }
                self.outdent();
                r
            }
            StatementType::FunctionDeclaration { name, params, body } => {
                let mut r = self.p(&format!("FunctionDeclaration (name: '{}')", name));
                self.indent();
                r.push_str(&self.p(&format!("Params: {:?}", params)));
                r.push_str(&self.p("Body:"));
                self.indent();
                for inner_stmt in body {
                    r.push_str(&self.print_statement(inner_stmt));
                }
                self.outdent();
                self.outdent();
                r
            }
            StatementType::ForLoop { var_decl, start, end, body } => {
                let mut r = self.p("ForLoopStatement");
                self.indent();

                r.push_str(&self.p("Initializer:"));
                self.indent();
                r.push_str(&self.print_statement(var_decl));
                self.outdent();

                r.push_str(&self.p("Range Start:"));
                self.indent();
                r.push_str(&self.print_expression(start));
                self.outdent();

                r.push_str(&self.p("Range End:"));
                self.indent();
                r.push_str(&self.print_expression(end));
                self.outdent();

                r.push_str(&self.p("Body:"));
                self.indent();
                for inner_stmt in body {
                    r.push_str(&self.print_statement(inner_stmt));
                }
                self.outdent();

                self.outdent();
                r
            }
        }
    }

    pub fn print_expression(&mut self, expr: &Expression) -> String {
        let mut r = String::new();
        match &expr.expression_type {
            ExpressionType::Identifier(name) => r.push_str(&self.p(&format!("Identifier('{}')", name))),
            ExpressionType::Integer(val) => r.push_str(&self.p(&format!("Integer({})", val))),
            ExpressionType::Float(val) => r.push_str(&self.p(&format!("Float({})", val))),
            ExpressionType::String(val) => r.push_str(&self.p(&format!("String(\"{}\")", val))),
            ExpressionType::Boolean(val) => r.push_str(&self.p(&format!("Boolean({})", val))),

            ExpressionType::BinaryOp { left, op, right } => {
                r.push_str(&self.p(&format!("BinaryOp ({:?})", op)));
                self.indent();
                r.push_str(&self.print_expression(left));
                r.push_str(&self.print_expression(right));
                self.outdent();
            }
            ExpressionType::PropertyAccess { object, property } => {
                r.push_str(&self.p("PropertyAccess"));
                self.indent();

                r.push_str(&self.p("Object:"));
                self.indent();
                r.push_str(&self.print_expression(object));
                self.outdent();

                r.push_str(&self.p(&format!("Property: '{}'", property)));

                self.outdent();
            }
            ExpressionType::Call { callee, args } => {
                r.push_str(&self.p("Call"));
                self.indent();

                r.push_str(&self.p("Callee:"));
                self.indent();
                r.push_str(&self.print_expression(callee));
                self.outdent();

                if !args.is_empty() {
                    r.push_str(&self.p("Arguments:"));
                    self.indent();
                    for arg in args {
                        r.push_str(&self.print_expression(arg));
                    }
                    self.outdent();
                } else {
                    r.push_str(&self.p("Arguments: None"));
                }
                self.outdent();
            }
        }
        r
    }
}
