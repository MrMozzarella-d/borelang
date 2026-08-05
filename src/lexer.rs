use std::ptr::null;
use crate::token::{Token, TokenType};

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}
impl Lexer {
    pub fn new(source:String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 0,
            column: 0,
        }
    }
    fn advance(&mut self) -> char {
        if Lexer::is_at_end(self) { return char::from(0); }
        let char = self.source.as_bytes().to_vec()[self.current] as char;
        self.current += 1;
        self.column += 1;
        if char == '\n' {
            self.line += 1;
            self.column += 1;
        };
        char
    }
    fn peek(&self) -> char {
        if self.is_at_end() { return char::from(0); }
        self.source.as_bytes().to_vec()[self.current] as char
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return char::from(0); }
        self.source.as_bytes().to_vec()[self.current + 1] as char
    }
    fn is_at_end(&self) -> bool { self.current >= self.source.len() }
}
pub fn tokenize(code: String) -> Vec<Token> {
    let mut lexer = Lexer::new(code);
    let mut token_vec = Vec::new();
    while lexer.current < lexer.source.len() {
        let char = lexer.advance();
        match char {
            ' ' | '\n' |  '\r' |  '\t'   => continue;
            '='                     => token_vec.push(Token::)
        }
    };
}