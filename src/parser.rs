use crate::node::{get_type, Expression, Statement, Type};
use crate::token::{Token, TokenData};
use crate::error::SyntaxError;

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
    fn expect(&mut self, expected_type: TokenData) -> Option<&Token<'a>> {
        let token = self.advance();
        if token.token_data != expected_type {
            eprintln!("Expected type {:?}, but got type {:?} at {}:{}", expected_type, token.token_data, token.line, token.column);
            return None
        }
        Some(token)
    }
    // fn peek_expect(&self, expected_type: TokenData) -> Option<&Token<'a>> {
    //     if let Some(token) = self.peek() {
    //         if token.token_data != expected_type {
    //             eprintln!("Expected type {:?}, but got type {:?} at {}:{}", expected_type, token.token_data, token.line, token.column);
    //             return None
    //         }
    //         return Some(token)
    //     };
    //     None
    // }
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
    //
    // }
    fn expect_literal(&mut self) -> Result<&'a str, SyntaxError> {
        match self.advance() {
            Token { token_data: TokenData::Literal(value), .. } => Ok(value),
            _ => Err(SyntaxError::ExpectedLiteral),
        }

    }
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current)
    }
    // fn peek_next(&self) -> Option<&Token<'a>> {
    //     if self.current + 1 >= self.tokens.len() || self.tokens.get(self.current+1)?.token_data == TokenData::EOF {
    //         return None
    //     }
    //     self.tokens.get(self.current+1)
    // }
    //fn is_at_end(&mut self) -> bool {
    //    let peek = self.peek().unwrap();
    //    peek.token_type == TokenData::EOF
    //}
    pub(crate) fn parse(&mut self) -> Vec<Statement<'a>> {
        let mut statement_vec: Vec<Statement<'a>> = Vec::new();
        while self.current < self.tokens.len() {
            if let Some(token) = self.peek() {
                if token.token_data == TokenData::EOF {
                    break;
                }
            } else {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                statement_vec.push(stmt);
            } else {
                self.advance();
            }
        }
        statement_vec
    }
    fn parse_statement(&mut self) -> Option<Statement<'a>> {
        let token_type = self.peek()?.token_data;
        match token_type {
            TokenData::Literal(name) => {
                match name {
                    "fn" => {
                        self.parse_function()
                    },
                    "let" => {
                        self.parse_var(false)
                    },
                    "mut" => {
                        self.parse_var(true)
                    },
                    "for" => {
                        self.parse_for()
                    },
                    _ => {
                        Some(Statement::Expression(self.parse_expression(0)?))
                    },
                }
            },
            TokenData::Return => { self.parse_return() },
            _ => None,
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
    fn parse_expression(&mut self, min_importance: usize) -> Option<Expression<'a>> {
        let left = self.advance();
        let mut left_expr = match left.token_data {
            TokenData::IntegerLiteral(v) => Expression::Integer(v),
            TokenData::FloatLiteral(v) => Expression::Float(v),
            TokenData::StringLiteral(v) => Expression::String(v),
            TokenData::Literal(v) => Expression::Identifier(v),
            TokenData::BooleanLiteral(v) => Expression::Boolean(v),
            _ => {
                eprintln!("Syntax Error: Expected Integer, Float, Boolean, String or Literal at {}:{}, but got {:?}.", left.line, left.column, left.token_data);
                return None;
            }
        };
        while let Some(token) = self.peek().map(|t| t) {
            let next = token.token_data;
            match next {
                TokenData::Dot => {
                    let op_importance = self.get_importance(next);
                    if op_importance <= min_importance {
                        break;
                    }
                    self.advance(); // consume dot
                    let property = self.expect_literal().unwrap();
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
                    self.expect(TokenData::CloseParen);
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
        Some(left_expr)
    }

    fn parse_function(&mut self) -> Option<Statement<'a>> {
        self.advance(); // consume proc, to name
        let func_name = self.expect_literal().unwrap(); println!("  func {}", func_name);
        self.expect(TokenData::OpenParen);
        let mut params = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseParen {
                break;
            }
            let param = self.expect_literal().unwrap();
            params.push(param);
            if self.peek()?.token_data == TokenData::Comma {
                self.advance();
            }
        }
        self.expect(TokenData::CloseParen);
        self.expect(TokenData::OpenBody);
        let mut body: Vec<Statement<'a>> = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        }
        //self.expect(TokenData::CloseBody);
        self.advance();
        let stmt = Statement::FunctionDeclaration {
            name: func_name,
            params,
            body
        };
        Some(stmt)
    }
    
    fn parse_return(&mut self) -> Option<Statement<'a>> {
        println!("  return");
        self.advance(); // consume return kw
        if self.peek().map(|t| t.is_atomic())? { // because parse_expression would error if it doesnt get a value we check it here first
            let expr = self.parse_expression(0)?;           // (that whiny ass bitch)
            let stmt = Statement::Return(Some(expr));
            return Some(stmt)
        }
        let stmt = Statement::Return(None);
        Some(stmt)
    }

    fn parse_var(&mut self, mutable: bool) -> Option<Statement<'a>> {
        self.advance(); // consume let/mut
        let name = self.expect_literal().unwrap();
        let mut ty: Option<Type> = None;
        println!("  var {}", name);
        if self.peek().map(|t| t.token_data == TokenData::Colon)? { // vars can either have a set type or not
            self.expect(TokenData::Colon); // consume colon
            let literal = self.expect_literal().unwrap();
            ty = get_type(literal);
        }
        if self.peek().map(|t| t.token_data == TokenData::Equal)? {
            self.expect(TokenData::Equal); // consume equal
            let expr = self.parse_expression(0);
            let stmt = Statement::VariableDeclaration {
                name,
                ty,
                mutable,
                value: expr,
            };
            Some(stmt)
        } else {
            let stmt = Statement::VariableDeclaration {
                name,
                ty,
                mutable,
                value: None,
            };
            Some(stmt)
        }
    }

    fn parse_for(&mut self) -> Option<Statement<'a>> {
        self.advance(); // consume for
        let var_data = self.peek()?.token_data;
        let var_str: &'a str;
        match var_data { 
            TokenData::Literal(v) => {
                if v != "let" && v != "mut" { return None }
                var_str = v;
            },
            _ => return None,
        }
        let var_decl = self.parse_var(var_str == "mut")?;

        let in_str = self.expect_literal().unwrap();
        if in_str != "in" { return None }
        let start = self.parse_expression(0)?;
        self.expect(TokenData::Range);
        let end = self.parse_expression(0)?;
        self.expect(TokenData::OpenBody);
        let mut body = Vec::new();
        while self.current < self.tokens.len() {
            if self.peek()?.token_data == TokenData::CloseBody {
                break;
            }
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            } else {
                self.advance();
            }
        };
        let stmt = Statement::ForLoop {
            var_decl: Box::new(var_decl),
            start,
            end,
            body,
        };
        Some(stmt)
    }
}