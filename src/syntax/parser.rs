use crate::runtime::error::{SyntaxError, SyntaxErrorType};
use crate::syntax::node::{Expression, ExpressionType, Statement, StatementType, PRIMITIVES};
use crate::syntax::token::{Token, TokenData};
use crate::{Type, TypeRule};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    fn advance(&mut self) -> &Token {
        if let Some(t) = self.tokens.get(self.current) {
            self.current += 1;
            t
        } else {
            &self.tokens[self.tokens.len() - 1] // return eof
        }
    }
    fn expect(&mut self, expected_type: TokenData) -> Result<&Token, SyntaxError> {
        let token = self.advance();
        if token.token_data != expected_type {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::Expected(expected_type, token.token_data.clone()),
                line: token.line,
                column: token.column,
            });
        }
        Ok(token)
    }
    // fn expect_one_of(&mut self, expected_types: Vec<TokenData>) -> Option<&Token<'a>> {
    //     let token = self.advance();
    //     let mut got = false;
    //     for expected in expected_types.iter() {
    //         if token.token_data == *expected {
    //             got = true;
    //         };
    //     };
    //     if got {
    //         Some(token)
    //     } else {
    //         eprint!("Expected one of ");
    //         for exp in expected_types.iter() {
    //             eprint!("{:?}, ", *exp);
    //         }
    //         eprintln!("but got {:?} at {}:{}.", token.token_data, token.line, token.column);
    //         None
    //     }
    // }
    fn expect_literal(&mut self) -> Result<String, SyntaxError> {
        let adv = self.advance();
        match adv {
            Token {
                token_data: TokenData::Literal(value),
                ..
            } => Ok(value.to_owned()),
            _ => Err(SyntaxError {
                error_type: SyntaxErrorType::Expected(
                    TokenData::Literal("".to_string()),
                    adv.token_data.clone(),
                ),
                line: adv.line,
                column: adv.column,
            }),
        }
    }
    fn peek(&self) -> Result<&Token, SyntaxError> {
        if let Some(tk) = self.tokens.get(self.current) {
            Ok(tk)
        } else {
            Err(SyntaxError {
                error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
                line: 0,
                column: 0,
            })
        }
    }
    fn peek_next(&self) -> Result<&Token, SyntaxError> {
        if let Some(tk) = self.tokens.get(self.current + 1) {
            Ok(tk)
        } else {
            Err(SyntaxError {
                error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
                line: 0,
                column: 0,
            })
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, SyntaxError> {
        let mut statement_vec: Vec<Statement> = Vec::new();
        while self.current < self.tokens.len() {
            if let Ok(token) = self.peek() {
                if token.token_data == TokenData::EOF {
                    break;
                }
            } else {
                break;
            }
            let res = self.parse_statement();
            if res.is_err() {
                return Err(res.unwrap_err());
            } else {
                let stmt = res?;
                statement_vec.push(stmt);
            }
        }
        Ok(statement_vec)
    }
    fn parse_statement(&mut self) -> Result<Statement, SyntaxError> {
        if let Ok(tk) = self.peek() {
            let line = tk.line;
            let column = tk.column;
            let tk = tk.clone();
            match tk.token_data {
                TokenData::Literal(name) => match name.as_str() {
                    "fn" => self.parse_function(),
                    "let" => self.parse_var(true),
                    "const" => self.parse_var(false),
                    "for" => self.parse_for(),
                    "return" => self.parse_return(),
                    "import" => self.parse_import(),
                    "if" => self.parse_if(),
                    "while" => self.parse_while(),
                    _ => Ok(Statement {
                        statement_type: StatementType::Expression(self.parse_expression(0)?),
                        line,
                        column,
                    }),
                },
                _ => Err(SyntaxError {
                    error_type: SyntaxErrorType::Expected(
                        TokenData::Literal("".to_string()),
                        tk.token_data,
                    ),
                    line,
                    column,
                }),
            }
        } else {
            Err(SyntaxError {
                error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
                line: 0,
                column: 0,
            })
        }
    }
    fn get_importance(&self, token_type: TokenData) -> usize {
        match token_type {
            TokenData::Equal
            | TokenData::AddAssign
            | TokenData::SubAssign
            | TokenData::MulAssign
            | TokenData::DivAssign => 1,
            TokenData::Or => 2,
            TokenData::And => 3,
            TokenData::Equivalent
            | TokenData::GreaterThan
            | TokenData::GreaterThanEqual
            | TokenData::LessThan
            | TokenData::LessThanEqual
            | TokenData::NotEqual => 4,
            TokenData::Add | TokenData::Sub => 5,
            TokenData::Mul | TokenData::Div => 6,
            TokenData::Dot | TokenData::OpenParen => 10,
            _ => 0,
        }
    }
    fn parse_expression(&mut self, min_importance: usize) -> Result<Expression, SyntaxError> {
        let left = self.advance();
        let mut left_expr = match left.clone().token_data {
            TokenData::IntegerLiteral(v) => Expression {
                expression_type: ExpressionType::Integer(v),
                line: left.line,
                column: left.column,
            },
            TokenData::FloatLiteral(v) => Expression {
                expression_type: ExpressionType::Float(v),
                line: left.line,
                column: left.column,
            },
            TokenData::StringLiteral(v) => Expression {
                expression_type: ExpressionType::String(v),
                line: left.line,
                column: left.column,
            },
            TokenData::Literal(v) => Expression {
                expression_type: ExpressionType::Identifier(v),
                line: left.line,
                column: left.column,
            },
            TokenData::BooleanLiteral(v) => Expression {
                expression_type: ExpressionType::Boolean(v),
                line: left.line,
                column: left.column,
            },
            TokenData::OpenParen => {
                let inner = self.parse_expression(0)?;
                self.expect(TokenData::CloseParen)?;
                inner
            }
            TokenData::Sub
            | TokenData::Not => {
                let clone = left.clone();
                let right = self.parse_expression(20)?;
                let line = right.line; let column = right.column;
                Expression {
                    expression_type: ExpressionType::Unary {
                        op: clone,
                        right: Box::new(right),
                    },
                    line, column,
                }
            }
            _ => {
                return Err(SyntaxError {
                    error_type: SyntaxErrorType::ExpectedAtomic(left.token_data.clone()),
                    line: left.line,
                    column: left.column,
                });
            }
        };
        while let Ok(token) = self.peek().map(|t| t) {
            let next = token.token_data.clone();
            match next {
                TokenData::Dot => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    let d = self.expect(TokenData::Dot)?; // consume dot
                    let line = d.line;
                    let column = d.column;
                    let property = self.expect_literal()?;
                    let expr = Expression {
                        expression_type: ExpressionType::PropertyAccess {
                            object: Box::new(left_expr),
                            property,
                        },
                        line,
                        column,
                    };
                    left_expr = expr;
                }
                TokenData::OpenParen => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    self.advance();
                    let mut expr_vec = Vec::new();
                    while self.current < self.tokens.len() {
                        if self.peek()?.token_data == TokenData::CloseParen {
                            break;
                        }
                        let arg_expr = self.parse_expression(0)?;
                        expr_vec.push(arg_expr);

                        if self.peek()?.token_data == TokenData::Comma {
                            self.advance();
                        }
                    }
                    self.expect(TokenData::CloseParen)?;
                    let line = left_expr.line;
                    let column = left_expr.column;
                    let expr = Expression {
                        expression_type: ExpressionType::Call {
                            callee: Box::new(left_expr),
                            args: expr_vec,
                        },
                        line, column,
                    };
                    left_expr = expr;
                }
                _ => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    let op = self.advance().clone();
                    let line = op.line;
                    let column = op.column;
                    let right_expr = self.parse_expression(op_importance)?;
                    match op.token_data {
                        TokenData::AddAssign | TokenData::DivAssign | TokenData::MulAssign | TokenData::SubAssign | TokenData::Equal => {
                            match &left_expr.expression_type {
                                ExpressionType::Identifier(_) | ExpressionType::PropertyAccess { .. } => {
                                    left_expr = Expression {
                                        expression_type: ExpressionType::Assignment {
                                            target: Box::new(left_expr),
                                            op,
                                            value: Box::new(right_expr),
                                        },
                                        line, column,
                                    }
                                }
                                _ => return Err(SyntaxError {
                                    error_type: SyntaxErrorType::InvalidAssignTarget,
                                    line, column
                                })
                            }
                        }
                        _ => {
                            left_expr = Expression {
                                expression_type: ExpressionType::BinaryOp {
                                    left: Box::new(left_expr),
                                    op: op.clone(),
                                    right: Box::new(right_expr),
                                },
                                line, column,
                            }
                        }
                    }
                }
            }
        }
        Ok(left_expr)
    }
    fn parse_type(&mut self) -> Result<Type, SyntaxError> {
        let first = self.expect_literal()?;
        let mut path = vec![first];
        while self.peek()?.token_data == TokenData::Dot {
              let n = self.expect_literal()?;
            path.push(n);
        }
        let mut gens = Vec::new();
        if self.peek()?.token_data == TokenData::BracketLeft {
            while self.current < self.tokens.len() {
                let inner = self.parse_type()?;
                gens.push(TypeRule::Explicit(inner));
                if self.advance().token_data == TokenData::Comma {
                    continue
                }
                break
            };
            self.expect(TokenData::BracketRight)?;
        }
        if path.len() == 1 && gens.is_empty() {
            if let Some(ty) = PRIMITIVES.get(path[0].as_str()) {
                return Ok(ty.clone());
            }
        };
        Ok(Type::Unresolved {
            path,
            gens,
        })
    }
    fn parse_function(&mut self) -> Result<Statement, SyntaxError> {
        let start = self.advance(); // consume proc, to name
        let line = start.line;
        let column = start.column;
        let func_name = self.expect_literal()?;
        self.expect(TokenData::OpenParen)?;
        let mut params = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseParen {
                break;
            }
            let param = self.expect_literal()?;
            if self.peek()?.token_data == TokenData::Colon { // set type
                self.expect(TokenData::Colon)?;
                let ty = self.parse_type()?;
                if self.peek()?.token_data == TokenData::Equal { // check if default
                    self.advance();
                    let expr = self.parse_expression(0)?;
                    params.push((param, TypeRule::Explicit(ty), Some(expr)));
                } else {
                    params.push((param, TypeRule::Explicit(ty), None))
                }
            } else {
                params.push((param, TypeRule::Any, None));
            }
            if self.peek()?.token_data == TokenData::Comma {
                self.advance();
            }
        }
        self.expect(TokenData::CloseParen)?;
        let mut rtn = TypeRule::Any;
        if self.peek()?.token_data == TokenData::Arrow {
            self.advance();
            let ty = self.parse_type()?;
            rtn = TypeRule::Explicit(ty);
        }
        self.expect(TokenData::OpenBody)?;
        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            let res = self.parse_statement();
            if res.is_err() {
                return Err(res.unwrap_err());
            } else {
                let stmt = res?;
                body.push(stmt);
            }
        }
        self.expect(TokenData::CloseBody)?;
        let stmt = StatementType::FunctionDeclaration {
            name: func_name,
            params,
            body,
            rtn,
        };
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }

    fn parse_return(&mut self) -> Result<Statement, SyntaxError> {
        let kw = self.advance(); // consume return kw
        let line = kw.line;
        let column = kw.column;
        if self.peek().map(|t| t.is_atomic())? {
            // because parse_expression would error if it doesnt get a value we check it here first
            let expr = self.parse_expression(0)?; // (that whiny ass bitch)
            let stmt = StatementType::Return(Some(expr));
            return Ok(Statement {
                statement_type: stmt,
                line,
                column,
            });
        }
        let stmt = StatementType::Return(None);
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }

    fn parse_var(&mut self, mutable: bool) -> Result<Statement, SyntaxError> {
        let start = self.advance(); // consume let/mut
        let line = start.line;
        let column = start.column;
        let name = self.expect_literal()?;
        let mut type_rule = TypeRule::Any;
        if self.peek()?.token_data == TokenData::Colon {
            self.expect(TokenData::Colon)?; // consume colon
            let ty = self.parse_type()?;
            type_rule = TypeRule::Explicit(ty);
        }
        if self.peek()?.token_data == TokenData::Equal {
            self.expect(TokenData::Equal)?; // consume equal
            let expr = self.parse_expression(0)?;
            let stmt = StatementType::VariableDeclaration {
                name,
                mutable,
                type_rule,
                value: Some(expr),
            };
            Ok(Statement {
                statement_type: stmt,
                line,
                column,
            })
        } else {
            let stmt = StatementType::VariableDeclaration {
                name,
                mutable,
                type_rule,
                value: None,
            };
            Ok(Statement {
                statement_type: stmt,
                line,
                column,
            })
        }
    }

    fn parse_for(&mut self) -> Result<Statement, SyntaxError> {
        let f = self.advance(); // consume for
        let line = f.line;
        let column = f.column;
        let var_str = match &self.peek()?.token_data {
            TokenData::Literal(s) => s.to_string(),
            x => {
                return Err(SyntaxError {
                    error_type: SyntaxErrorType::Unexpected(x.clone()),
                    line: self.peek()?.line,
                    column: self.peek()?.column,
                });
            }
        };
        if var_str != "let" && var_str != "const" {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::ExpectedLiteral("let or const".to_string(), var_str),
                line: self.peek()?.line,
                column: self.peek()?.column,
            });
        }
        let var_decl = Box::new(self.parse_var(var_str == "let")?);

        let in_str = self.expect_literal()?;
        if in_str != "in" {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::ExpectedLiteral(
                    String::from("in"),
                    in_str.to_string(),
                ),
                line: self.peek()?.line,
                column: self.peek()?.column,
            });
        }
        let start = self.parse_expression(0)?;
        self.expect(TokenData::Range)?;
        let end = self.parse_expression(0)?;
        // let mut step = Expression {
        //     expression_type: ExpressionType::Integer(1),
        //     line: end.line,
        //     column: end.column,
        // };
        // if self.peek()?.token_data == TokenData::Literal("with".to_string()) { 
        //     self.expect_literal()?;
        //     let expr = self.parse_expression(0)?;
        //     step = expr;
        // }
        self.expect(TokenData::OpenBody)?;
        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            let res = self.parse_statement();
            if res.is_err() {
                return Err(res.unwrap_err());
            } else {
                let stmt = res?;
                body.push(stmt);
            }
        }

        self.expect(TokenData::CloseBody)?;
        let stmt = StatementType::ForLoop {
            var_decl,
            start,
            end,
            // step,
            body,
        };
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }

    fn parse_import(&mut self) -> Result<Statement, SyntaxError> {
        let i = self.advance(); // consume import
        let line = i.line;
        let column = i.column;
        let mut mods = Vec::new();
        // 'parent: while self.current < self.tokens.len() {
        //     match self.peek()?.token_data {
        //         TokenData::Comma => {
        //             self.expect(TokenData::Comma)?;
        //             let i = self.expect_literal()?;
        //             if i.as_str() == "import" {
        //                 return Err(SyntaxError {
        //                     error_type: SyntaxErrorType::ExpectedLiteral(
        //                         "module name".to_string(),
        //                         i,
        //                     ),
        //                     line,
        //                     column,
        //                 });
        //             }
        //             mods.push(i);
        //         }
        //         _ => break 'parent,
        //     }
        // }
        mods.push(self.expect_literal()?);
        let stmt = StatementType::Import { mods };
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }
    fn parse_if(&mut self) -> Result<Statement, SyntaxError> {
        let i = self.advance(); // if
        let line = i.line;
        let column = i.column;
        let condition = self.parse_expression(0)?;
        self.expect(TokenData::OpenBody)?;
        let mut then_body = Vec::new();
        while self.current < self.tokens.len() {
            let stmt = self.parse_statement()?;
            then_body.push(stmt);
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
        };
        if self.peek_next()?.token_data == TokenData::Literal("else".into()) {
            self.advance(); self.advance(); // advance to else, then to next token
            self.expect(TokenData::OpenBody)?;
            let mut else_body = Vec::new();
            while self.current < self.tokens.len() {
                let stmt = self.parse_statement()?;
                else_body.push(stmt);
                if self.peek()?.token_data == TokenData::CloseBody {
                    break;
                }
            };
            self.expect(TokenData::CloseBody)?;
            let stmt = StatementType::If {
                condition,
                then_branch: then_body,
                else_branch: Some(else_body),
            };
            Ok(Statement {
                statement_type: stmt,
                line,
                column,
            })
        } else {
            self.advance();
            let stmt = StatementType::If {
                condition,
                then_branch: then_body,
                else_branch: None,
            };
            Ok(Statement {
                statement_type: stmt,
                line,
                column,
            })
        }
    }
    fn parse_while(&mut self) -> Result<Statement, SyntaxError> {
        let w = self.advance();
        let line = w.line;
        let column = w.column;
        let condition = self.parse_expression(0)?;
        self.expect(TokenData::OpenBody)?;
        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break
            }
            body.push(self.parse_statement()?);
        };
        self.expect(TokenData::CloseBody)?;
        let statement_type = StatementType::WhileLoop {
            condition,
            body,
        };
        Ok(Statement {
            statement_type,
            line,
            column,
        })
    }
}
