use crate::error::{SyntaxError, SyntaxErrorType};
use crate::node::{Expression, ExpressionType, Statement, StatementType};
use crate::token::{Token, TokenData};

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
            // if tk.token_data == TokenData::EOF {
            //     return Err(SyntaxError{
            //         error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
            //         line: tk.line,
            //         column: tk.column,
            //     })
            // }
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
    pub(crate) fn parse(&mut self) -> Result<Vec<Statement>, SyntaxError> {
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
                    "let" => self.parse_var(false),
                    "mut" => self.parse_var(true),
                    "for" => self.parse_for(),
                    "return" => self.parse_return(),
                    "import" => self.parse_import(),
                    "if" => self.parse_if(),
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
            TokenData::Equivalent => 2,
            TokenData::Add | TokenData::Sub => 3,
            TokenData::Mul | TokenData::Div => 4,
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
                        line,
                        column,
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
                    left_expr = Expression {
                        expression_type: ExpressionType::BinaryOp {
                            left: Box::new(left_expr),
                            op: op.clone(),
                            right: Box::new(right_expr),
                        },
                        line,
                        column,
                    }
                }
            }
        }
        Ok(left_expr)
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
            params.push(param);
            if self.peek()?.token_data == TokenData::Comma {
                self.advance();
            }
        }
        self.expect(TokenData::CloseParen)?;
        self.expect(TokenData::OpenBody)?;
        let mut body: Vec<Statement> = Vec::new();
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
        //self.expect(TokenData::CloseBody);
        self.advance();
        let stmt = StatementType::FunctionDeclaration {
            name: func_name,
            params,
            body,
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
        // if self.peek().map(|t| t.token_data == TokenData::Colon)? { // no more
        //     // dynamic / explicit 
        //     self.expect(TokenData::Colon)?; // consume colon
        //     let literal = self.expect_literal()?;
        // }
        if self.peek().map(|t| t.token_data == TokenData::Equal)? {
            self.expect(TokenData::Equal)?; // consume equal
            let expr = self.parse_expression(0)?;
            let stmt = StatementType::VariableDeclaration {
                name,
                mutable,
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
        if var_str != "let" && var_str != "mut" {
            return Err(SyntaxError {
                error_type: SyntaxErrorType::ExpectedLiteral("let or mut".to_string(), var_str),
                line: self.peek()?.line,
                column: self.peek()?.column,
            });
        }
        let var_decl = Box::new(self.parse_var(var_str == "mut")?);

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
        'parent: while self.current < self.tokens.len() {
            match self.peek()?.token_data {
                TokenData::Comma => {
                    self.expect(TokenData::Comma)?;
                    let i = self.expect_literal()?;
                    if i.as_str() == "import" {
                        return Err(SyntaxError {
                            error_type: SyntaxErrorType::ExpectedLiteral(
                                "module name".to_string(),
                                i,
                            ),
                            line,
                            column,
                        });
                    }
                    mods.push(i);
                }
                _ => break 'parent,
            }
        }
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
}

//fn get_type(literal: &String) -> Type {
//    match literal.as_str() {
//        "int" => Type::Int,
//        "float" => Type::Float,
//        "str" => Type::Str,
//        "bool" => Type::Bool,
//        "null" => Type::Null,
//        other => Type::Unresolved(other.to_string()),
//    }
//}
