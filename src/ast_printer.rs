use crate::node::{Statement, Expression};

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
        match stmt {
            Statement::VariableDeclaration { mutable, name, ty, value } => {
                let mut r = self.p(&format!("VariableDeclaration (name: '{}', mut: {})", name, mutable));
                self.indent();
                if let Some(t) = ty {
                    r.push_str(&self.p(&format!("Type: {:?}", t)));
                }
                if let Some(v) = value {
                    r.push_str(&self.p("Value:"));
                    self.indent();
                    r.push_str(&self.print_expression(v));
                    self.outdent();
                }
                self.outdent();
                r
            }
            Statement::Assignment { target, value } => {
                let mut r = self.p("Assignment");
                self.indent();
                r.push_str(&self.p("Target:"));
                self.indent();
                r.push_str(&self.print_expression(target));
                self.outdent();
                r.push_str(&self.p("Value:"));
                self.indent();
                r.push_str(&self.print_expression(value));
                self.outdent();
                self.outdent();
                r
            }
            Statement::Expression(expr) => {
                let mut r = self.p("ExpressionStatement");
                self.indent();
                r.push_str(&self.print_expression(expr));
                self.outdent();
                r
            }
            Statement::Return(value) => {
                let mut r = self.p("Return");
                if let Some(expr) = value {
                    self.indent();
                    r.push_str(&self.print_expression(expr));
                    self.outdent();
                }
                r
            }
            Statement::If { condition, then_branch, else_branch } => {
                let mut r = self.p("IfStatement");
                self.indent();
                r.push_str(&self.p("Condition:"));
                self.indent();
                r.push_str(&self.print_expression(condition));
                self.outdent();

                r.push_str(&self.p("Then:"));
                self.indent();
                for stmt in then_branch {
                    r.push_str(&self.print_statement(stmt));
                }
                self.outdent();

                if let Some(else_stmts) = else_branch {
                    r.push_str(&self.p("Else:"));
                    self.indent();
                    for stmt in else_stmts {
                        r.push_str(&self.print_statement(stmt));
                    }
                    self.outdent();
                }
                self.outdent();
                r
            }
            Statement::FunctionDeclaration { name, params, body } => {
                let mut r = self.p(&format!("FunctionDeclaration (name: '{}')", name));
                self.indent();
                r.push_str(&self.p(&format!("Params: {:?}", params)));
                r.push_str(&self.p("Body:"));
                self.indent();
                for stmt in body {
                    r.push_str(&self.print_statement(stmt));
                }
                self.outdent();
                self.outdent();
                r
            }
        }
    }

    pub fn print_expression(&mut self, expr: &Expression) -> String {
        let mut r = String::new();
        match expr {
            Expression::Identifier(name) => r.push_str(&self.p(&format!("Identifier('{}')", name))),
            Expression::Integer(val) => r.push_str(&self.p(&format!("Integer({})", val))),
            Expression::Float(val) => r.push_str(&self.p(&format!("Float({})", val))),
            Expression::String(val) => r.push_str(&self.p(&format!("String(\"{}\")", val))),
            Expression::Boolean(val) => r.push_str(&self.p(&format!("Boolean({})", val))),

            Expression::BinaryOp { left, op, right } => {
                r.push_str(&self.p(&format!("BinaryOp ({:?})", op)));
                self.indent();
                r.push_str(&self.print_expression(left));
                r.push_str(&self.print_expression(right));
                self.outdent();
            }
            Expression::PropertyAccess { object, property } => {
                r.push_str(&self.p("PropertyAccess"));
                self.indent();

                r.push_str(&self.p("Object:"));
                self.indent();
                r.push_str(&self.print_expression(object));
                self.outdent();

                r.push_str(&self.p("Property:"));
                self.indent();
                r.push_str(&self.print_expression(property));
                self.outdent();

                self.outdent();
            }
            Expression::Call { callee, args } => {
                r.push_str(&self.p(&format!("Call (callee: '{}')", callee)));
                self.indent();

                if !args.is_empty() {
                    for arg in args {
                        r.push_str(&self.print_expression(arg));
                    }
                }

                self.outdent();
            }
            Expression::Object(fields) => {
                r.push_str(&self.p("Object"));
                self.indent();
                for (name, expr) in fields {
                    r.push_str(&self.p(&format!("Field '{}':", name)));
                    self.indent();
                    r.push_str(&self.print_expression(expr));
                    self.outdent();
                }
                self.outdent();
            }
        }
        r
    }
}
