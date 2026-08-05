use crate::node::{Expression, Statement};
use crate::token::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, current: 0 } }

    fn advance(&mut self) -> &Token {
        let t = self.tokens.get(self.current).unwrap();
        self.current += 1;
        t
    }
    fn expect(&mut self, expected_type: TokenType) -> Option<&Token> {
        let token = self.advance();
        if token.token_type != expected_type {
            eprintln!("Expected type {:?}, got type {:?} at {}:{}", expected_type, token.token_type, token.line, token.column);
            return None
        }
        Some(token)
    }
    fn peek_expect(&mut self, expected_type: TokenType) -> Option<&Token> {
        let token = self.peek();
        if token.unwrap().token_type != expected_type {
            eprintln!("Expected type {:?}, got type {:?} at {}:{}", expected_type, token.unwrap().token_type, token.unwrap().line, token.unwrap().column);
            return None
        }
        token
    }
    fn expect_one_of(&mut self, expected_types: Vec<TokenType>) -> Option<&Token> {
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
            eprintln!("got {:?}.", token.token_type);
            None
        }

    }
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.current)
    }
    fn peek_next(&mut self) -> Option<&Token> {
        if self.current + 1 >= self.tokens.len() || self.tokens.get(self.current+1)?.token_type == TokenType::EOF {
            return None
        }
        self.tokens.get(self.current+1)
    }
    //fn is_at_end(&mut self) -> bool {
    //    let peek = self.peek().unwrap();
    //    peek.token_type == TokenType::EOF
    //}
    pub(crate) fn parse(&mut self) -> Vec<Statement> {
        let mut statement_vec = Vec::new();
        while self.current < self.tokens.len() {
            let token = self.advance();
            match token.token_type {
                TokenType::EOF => break,
                TokenType::KwProc => {
                    let name_token = self.expect(TokenType::Identifier).unwrap();
                    let proc_name = name_token.value.clone();
                    self.expect(TokenType::ParenLeft);
                    let mut params = Vec::new();
                    while self.current < self.tokens.len() {
                        if self.peek().unwrap().token_type == TokenType::ParenRight {
                            break;
                        }
                        let param = self.expect(TokenType::Identifier).unwrap();
                        let param_name = param.value.clone();
                        params.push(param_name);
                        if self.peek().unwrap().token_type != TokenType::ParenRight { // expect a comma and another identifier if there is no other parentheses
                            self.expect(TokenType::Comma);
                            self.peek_expect(TokenType::Identifier);
                        }
                    }
                    self.advance();
                    self.expect(TokenType::BraceLeft);
                    let mut body = Vec::new();
                    while self.current < self.tokens.len() {
                        if self.peek().unwrap().token_type == TokenType::BraceRight {
                            break;
                        }
                        let statement = self.parse_statement().unwrap();
                        body.push(statement);
                    }

                    statement_vec.push(Statement::new_procedure(
                        proc_name,
                        params,
                        body
                    ))
                },
                _ => {
                    let stmt = self.parse_statement().unwrap();
                    statement_vec.push(stmt);
                },
            }
        }
        statement_vec
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        let token = self.advance();
        match token.token_type {
            TokenType::Identifier => {
                let value = token.value.clone();
                self.expect(TokenType::ParenLeft); // expect it to be a function
                let mut expr_vec = Vec::new();
                while self.current < self.tokens.len() {
                    if self.peek().unwrap().token_type == TokenType::ParenRight {
                        break;
                    }
                    let arg_token = self.expect_one_of(vec![TokenType::Identifier, TokenType::Number]).unwrap();
                    let arg_expr = self.parse_expression(arg_token).unwrap();
                    expr_vec.push(arg_expr);
                };
                let stmt = Statement::new_call(value, expr_vec);
                Some(stmt)
            },
            _ => None,
        }
    }
    fn parse_expression(&mut self, token: &Token) -> Option<Expression> {
        match token.token_type {
            TokenType::Identifier => {
                let value = token.value.clone();
                Some(Expression::new_identifier(value))
            },
            TokenType::Number => Some(Expression::new_number(token.value.parse().unwrap())),
            TokenType::StringLiteral => {
                let value = token.value.clone();
                Some(Expression::new_string_literal(value))
            },
            _ => {
                eprintln!("Expected type of identifier, number or string literal when parsing expression, got {:?}.", token.token_type);
                None
            }
        }
    }
}