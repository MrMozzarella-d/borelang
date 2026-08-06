use crate::node::{Expression, Statement};
use crate::token::{Token, TokenType};

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
        let t = self.tokens.get(self.current).unwrap();
        self.current += 1;
        t
    }
    fn expect(&mut self, expected_type: TokenType) -> Option<&Token<'a>> {
        let token = self.advance();
        if token.token_type != expected_type {
            eprintln!("Expected type {:?}, got type {:?} at {}:{}", expected_type, token.token_type, token.line, token.column);
            return None
        }
        Some(token)
    }
    fn peek_expect(&self, expected_type: TokenType) -> Option<&Token<'a>> {
        let token = self.peek();
        if token.unwrap().token_type != expected_type {
            eprintln!("Expected type {:?}, got type {:?} at {}:{}", expected_type, token.unwrap().token_type, token.unwrap().line, token.unwrap().column);
            return None
        }
        token
    }
    fn expect_one_of(&mut self, expected_types: Vec<TokenType>) -> Option<&Token<'a>> {
        let token = self.advance();
        let mut got = false;
        for expected in expected_types.iter() {
            if token.token_type == *expected {
                got = true;
            };
        };
        if got {
            Some(token)
        } else {
            eprint!("Expected one of ");
            for exp in expected_types.iter() {
                eprint!("{:?}, ", *exp);
            }
            eprintln!("but got {:?} at {}:{}.", token.token_type, token.line, token.column);
            None
        }

    }
    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current)
    }
    fn peek_next(&self) -> Option<&Token<'a>> {
        if self.current + 1 >= self.tokens.len() || self.tokens.get(self.current+1)?.token_type == TokenType::EOF {
            return None
        }
        self.tokens.get(self.current+1)
    }
    //fn is_at_end(&mut self) -> bool {
    //    let peek = self.peek().unwrap();
    //    peek.token_type == TokenType::EOF
    //}
    pub(crate) fn parse(&mut self) -> Vec<Statement<'a>> {
        let mut statement_vec: Vec<Statement<'a>> = Vec::new();
        while self.current < self.tokens.len() {
            if let Some(token) = self.peek() {
                if token.token_type == TokenType::EOF {
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
        let token_type = self.peek()?.token_type;
        match token_type {
            TokenType::KwProc => {
                self.advance(); // consume proc, to name
                let name_token = self.expect(TokenType::Identifier).unwrap();
                let proc_name = name_token.value;
                self.expect(TokenType::ParenLeft);
                let mut params = Vec::new();
                while self.current < self.tokens.len() {
                    if self.peek().unwrap().token_type == TokenType::ParenRight {
                        break;
                    }
                    let param = self.expect(TokenType::Identifier).unwrap();
                    params.push(param.value);
                    if self.peek().unwrap().token_type == TokenType::Comma {
                        self.advance();
                    }
                }
                self.expect(TokenType::ParenRight);
                self.expect(TokenType::BraceLeft);
                let mut body: Vec<Statement<'a>> = Vec::new();
                while self.current < self.tokens.len() {
                    if self.peek().unwrap().token_type == TokenType::BraceRight {
                        break;
                    }
                    if let Some(stmt) = self.parse_statement() {
                        body.push(stmt);
                    } else {
                        self.advance();
                    }
                }
                self.expect(TokenType::BraceRight);
                let stmt = Statement::new_procedure(proc_name, params, body);
                Some(stmt)
            },
            TokenType::Identifier => {
                let callee_token = self.advance(); // consume identifier
                let callee = callee_token.value;
                self.expect(TokenType::ParenLeft); // expect it to be a function, we are now on the first letter of the function args
                let mut expr_vec = Vec::new();
                while self.current < self.tokens.len() {
                    if self.peek().unwrap().token_type == TokenType::ParenRight {
                        break;
                    }
                    let arg_expr = self.parse_expression(0).unwrap();
                    expr_vec.push(arg_expr);

                    if self.peek().unwrap().token_type == TokenType::Comma {
                        self.advance();
                    }
                };
                self.expect(TokenType::ParenRight);

                let stmt = Statement::new_call(callee, expr_vec);
                Some(stmt)
            },
            _ => None,
        }
    }
    fn get_importance(&self, token_type: TokenType) -> usize {
        match token_type {
            TokenType::Equal | TokenType::PlusEqual | TokenType::MinusEqual | TokenType::MultiplyEqual | TokenType::DivideEqual => 1,
            TokenType::CompEqual => 2,
            TokenType::Plus | TokenType::Minus => 3,
            TokenType::Multiply | TokenType::Divide => 4,
            _ => 0,
        }
    }
    fn parse_expression(&mut self, min_importance: usize) -> Option<Expression<'a>> {
        let left = self.advance();
        let mut left_expr = match left.token_type {
            TokenType::StackPointReference | TokenType::StackAliasReference => Expression::StackReference(*left),
            TokenType::Number => Expression::NumberLiteral(left.value.parse().unwrap()),
            TokenType::StringLiteral => Expression::StringLiteral(left.value.parse().unwrap()),
            TokenType::Identifier => {
                let mut id = left.value;
                if self.peek().map(|t| t.token_type) == Some(TokenType::ParenLeft) { // function
                    self.advance(); // consume the parenLeft
                    let mut args = Vec::new();
                    while self.current < self.tokens.len() {
                        if self.peek().map(|t| t.token_type) == Some(TokenType::ParenRight) {
                            break;
                        }
                        let arg_expr = self.parse_expression(0).unwrap();
                        args.push(arg_expr);

                        if self.peek().unwrap().token_type == TokenType::Comma {
                            self.advance();
                        }
                    }
                    self.expect(TokenType::ParenRight);
                    Expression::Call {
                        callee: id,
                        args
                    }
                } else {
                    Expression::Identifier(id)
                }
            },
            _ => {
                eprint!("Syntax Error: Expected Number, String, Identifier, or Stack Reference at {}:{}, but got {:?}.", left.line, left.column, left.token_type);
                return None;
            }
        };
        while let Some(next) = self.peek().map(|t| t.token_type) {
            let op_importance = self.get_importance(next);
            if op_importance <= min_importance {
                break;
            }
            let op = *self.advance();
            let right_expr = self.parse_expression(op_importance).unwrap();
            left_expr = Expression::new_binary_op(
                Box::new(left_expr),
                op,
                Box::new(right_expr),
            )
        };
        Some(left_expr)
    }
}