use crate::node::{get_type, Expression, Statement, Type, StatementType};
use crate::token::{Token, TokenData};
use crate::error::{SyntaxError, SyntaxErrorType};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }
    fn advance(&mut self) -> &Token<'a> {
        if let Some(t) = self.tokens.get(self.current) {
            self.current += 1;
            t
        } else {
            &self.tokens[self.tokens.len() - 1] // return eof
        }
    }
    fn expect<'b>(&'b mut self, expected_type: TokenData<'b>) -> Result<&Token<'a>, SyntaxError> {
        let token = self.advance();
        let ty = token.token_data;
        if ty != expected_type {
            return Err(SyntaxError{
                error_type: SyntaxErrorType::Expected(expected_type, ty),
                line: token.line,
                column: token.column,
            })
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
    fn expect_literal(&mut self) -> Result<&'a str, SyntaxError> {
        let adv = self.advance();
        match adv {
            Token { token_data: TokenData::Literal(value), .. } => Ok(value),
            _ => Err(SyntaxError{
                error_type: SyntaxErrorType::Expected(TokenData::Literal(""), adv.token_data),
                line: adv.line,
                column: adv.column,
            }),
        }

    }
    fn peek(&self) -> Result<&Token<'a>, SyntaxError> {
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
            Err(SyntaxError{
                error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
                line: 0,
                column: 0,
            })
        }
    }
    pub(crate) fn parse(&mut self) -> Result<Vec<Statement<'a>>, SyntaxError> {
        let mut statement_vec: Vec<Statement<'a>> = Vec::new();
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
                return Err(res.unwrap_err())
            } else {
                let stmt = res?;
                statement_vec.push(stmt);
            }
        }
        Ok(statement_vec)
    }
    fn parse_statement(&mut self) -> Result<Statement<'a>, SyntaxError> {
        if let Ok(tk) = self.peek() {
            let line = tk.line;
            let column = tk.column;
            match tk.token_data {
                TokenData::Literal(name) => {
                    match name {
                        "fn" => self.parse_function(),
                        "let" => self.parse_var(false),
                        "mut" => self.parse_var(true),
                        "for" => self.parse_for(),
                        _ => {
                            Ok(Statement {
                                statement_type: StatementType::Expression(self.parse_expression(0)?),
                                line,
                                column,
                            })
                        },
                    }
                },
                TokenData::Return => { self.parse_return() },
                _ => {
                    Err(SyntaxError{
                        error_type: SyntaxErrorType::Expected(TokenData::Literal(""), tk.token_data),
                        line,
                        column,
                    })
                },
            }
        } else {
            Err(SyntaxError{
                error_type: SyntaxErrorType::Unexpected(TokenData::EOF),
                line: 0,
                column: 0,
            })
        }
    }
    fn get_importance(&self, token_type: TokenData) -> usize {
        match token_type {
            TokenData::Equal | TokenData::AddAssign | TokenData::SubAssign | TokenData::MulAssign | TokenData::DivAssign => 1,
            TokenData::Equivalent => 2,
            TokenData::Add | TokenData::Sub => 3,
            TokenData::Mul | TokenData::Div => 4,
            TokenData::Dot | TokenData::OpenParen => 10,
            _ => 0,
        }
    }
    fn parse_expression(&mut self, min_importance: usize) -> Result<Expression<'a>, SyntaxError> {
        let left = self.advance();
        let mut left_expr = match left.token_data {
            TokenData::IntegerLiteral(v) => Expression::Integer(v),
            TokenData::FloatLiteral(v) => Expression::Float(v),
            TokenData::StringLiteral(v) => Expression::String(v),
            TokenData::Literal(v) => Expression::Identifier(v),
            TokenData::BooleanLiteral(v) => Expression::Boolean(v),
            _ => {
                return Err(SyntaxError{
                    error_type: SyntaxErrorType::ExpectedAtomic(left.token_data),
                    line: left.line,
                    column: left.column,
                });
            }
        };
        while let Ok(token) = self.peek().map(|t| t) {
            let next = token.token_data;
            match next {
                TokenData::Dot => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    self.advance(); // consume dot
                    let property = self.expect_literal()?;
                    let expr = Expression::PropertyAccess {
                        object: Box::new(left_expr),
                        property,
                    };
                    left_expr = expr;
                },
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
                    let expr = Expression::Call {
                        callee: Box::new(left_expr),
                        args: expr_vec,
                    };
                    left_expr = expr;
                },
                _ => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    let op = *self.advance();
                    let right_expr = self.parse_expression(op_importance)?;
                    left_expr = Expression::BinaryOp {
                        left: Box::new(left_expr),
                        op,
                        right: Box::new(right_expr),
                    }
                }
            }

        };
        Ok(left_expr)
    }

    fn parse_function(&mut self) -> Result<Statement<'a>, SyntaxError> {
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
        let mut body: Vec<Statement<'a>> = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            if let Ok(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        }
        //self.expect(TokenData::CloseBody);
        self.advance();
        let stmt = StatementType::FunctionDeclaration {
            name: func_name,
            params,
            body
        };
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }
    
    fn parse_return(&mut self) -> Result<Statement<'a>, SyntaxError> {
        let kw = self.advance(); // consume return kw
        let line = kw.line;
        let column = kw.column;
        if self.peek().map(|t| t.is_atomic())? { // because parse_expression would error if it doesnt get a value we check it here first
            let expr = self.parse_expression(0)?;           // (that whiny ass bitch)
            let stmt = StatementType::Return(Some(expr));
            return Ok(Statement{
                statement_type: stmt,
                line,
                column,
            })
        }
        let stmt = StatementType::Return(None);
        Ok(Statement {
            statement_type: stmt,
            line,
            column,
        })
    }

    fn parse_var(&mut self, mutable: bool) -> Result<Statement<'a>, SyntaxError> {
        let start = self.advance(); // consume let/mut
        let line = start.line;
        let column = start.column;
        let name = self.expect_literal()?;
        let mut ty: Option<Type> = None;
        if self.peek().map(|t| t.token_data == TokenData::Colon)? { // vars can either have a set type or not
            self.expect(TokenData::Colon)?; // consume colon
            let literal = self.expect_literal()?;
            ty = get_type(literal);
        }
        if self.peek().map(|t| t.token_data == TokenData::Equal)? {
            self.expect(TokenData::Equal)?; // consume equal
            let expr = self.parse_expression(0)?;
            let stmt = StatementType::VariableDeclaration {
                name,
                ty,
                mutable,
                value: Some(expr),
            };
            Ok(Statement {
                statement_type: stmt,
                line,
                column,
            })
        } else {
            if ty.is_none() {
                return Err(SyntaxError{
                    error_type: SyntaxErrorType::NullVariableWithoutType(name.to_string()),
                    line,
                    column,
                })
            }
            let stmt = StatementType::VariableDeclaration {
                name,
                ty,
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

    fn parse_for(&mut self) -> Result<Statement<'a>, SyntaxError> {
        let f = self.advance(); // consume for
        let line = f.line;
        let column = f.column;
        let var_str = match &self.tokens[self.current].token_data {
            TokenData::Literal(s) => s.to_string(),
            _ => return Err(SyntaxError{
                error_type: SyntaxErrorType::Expected(TokenData::Literal(""), self.peek()?.token_data),
                line: self.peek()?.line,
                column: self.peek()?.column,
            })
        };
        if var_str != "let" && var_str != "mut" {
            return Err(SyntaxError{
                error_type: SyntaxErrorType::ExpectedLiteral("let or mut".to_string(), var_str),
                line: self.peek()?.line,
                column: self.peek()?.column,
            })
        }
        let var_decl = Box::new(self.parse_var(var_str == "mut")?);

        let in_str = self.expect_literal()?;
        if in_str != "in" { return Err(SyntaxError{
            error_type: SyntaxErrorType::ExpectedLiteral(String::from("in"), in_str.to_string()),
            line: self.peek()?.line,
            column: self.peek()?.column,
        }) }
        let start = self.parse_expression(0)?;
        self.expect(TokenData::Range)?;
        let end = self.parse_expression(0)?;
        self.expect(TokenData::OpenBody)?;
        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            if let Ok(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        };
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
}