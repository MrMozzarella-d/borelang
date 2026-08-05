use crate::token;
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
            line: 1,
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
            self.column = 1;
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
        if char.is_alphabetic() || char == '_' {
            let start_pos = lexer.current - char.len_utf8();
            while lexer.current < lexer.source.len() {
                let next_char = lexer.peek();
                if next_char.is_alphanumeric() || next_char == '_' {
                    lexer.advance();
                } else {
                    break;
                }
            }
            let end_pos = lexer.current;
            let identifier = &lexer.source[start_pos..end_pos];
            if token::KEYWORDS.contains_key(identifier) {
                token_vec.push(Token::new(
                    *token::KEYWORDS.get(identifier).unwrap(),
                    identifier,
                    lexer.line,
                    lexer.column,
                ));
            } else {
                token_vec.push(Token::new(
                    TokenType::Identifier,
                    identifier,
                    lexer.line,
                    lexer.column,
                ));
            }
            continue;
        }
        if char.is_numeric() {
            let start_pos = lexer.current - char.len_utf8();
            while lexer.current < lexer.source.len() {
                let next = lexer.peek();
                if next.is_numeric() {
                    lexer.advance();
                } else {
                    break;
                }
            }   
            let end_pos = lexer.current;
            let val = &lexer.source[start_pos..end_pos];
            token_vec.push(Token::new(
                TokenType::Number,
                val,
                lexer.line,
                lexer.column,
            ))
        }
        
        match char {
            ' ' | '\n' |  '\r' |  '\t'  => { continue; },
            '@'                     => {
                let peek_next = lexer.peek();
                if peek_next == '[' { // alias reference
                    let _ = lexer.advance();
                    let start_pos = lexer.current - char.len_utf8();;
                    while lexer.current < lexer.source.len() {
                        let next_char = lexer.peek();
                        if next_char.is_alphanumeric() || next_char == '_' {
                            lexer.advance();
                        } else {
                            break;
                        }
                    }
                    let end_pos = lexer.current;
                    _ = lexer.advance(); // consume ]
                    let val = &lexer.source[start_pos..end_pos];
                    token_vec.push(Token::new(
                        TokenType::StackAliasReference,
                        val,
                        lexer.line,
                        lexer.column,
                    ))
                } else if peek_next.is_numeric() {
                    let start_pos = lexer.current;
                    while lexer.current < lexer.source.len() {
                        let char = lexer.peek();
                        if !char.is_numeric() {
                            break;
                        } else {
                            lexer.advance();
                        }
                    }
                    let end_pos = lexer.current;
                    let val = &lexer.source[start_pos..end_pos];
                    token_vec.push(Token::new(
                        TokenType::StackPointReference,
                        val,
                        lexer.line,
                        lexer.column,
                    ))
                } else {
                    eprintln!("Syntax Error: Unrecognized character for stack point reference '{}'", char);
                }
            },
            '+' | '/' | '*' | '=' | '-' => {
                match lexer.peek_next() {
                    '=' => {
                        let comb_string = format!("{char}=");
                        let comb = comb_string.as_str();
                        token_vec.push(Token::new(
                            *token::OPERATORS.get(comb).unwrap(),
                            comb,
                            lexer.line,
                            lexer.column,
                        ))
                    }
                    _ => {
                        let string = char.to_string();
                        let str = string.as_str();
                        token_vec.push(Token::new(
                            *token::OPERATORS.get(str).unwrap(),
                            str,
                            lexer.line,
                            lexer.column,
                        ))
                    }
                }
            },
            '"'                 => {
                let start_pos = lexer.current;
                while lexer.current < lexer.source.len() {
                    let next = lexer.peek();
                    if next == '"' {
                        break;
                    }
                    lexer.advance();
                }
                let end_pos = lexer.current - 1;
                let text = &lexer.source[start_pos..end_pos];
                token_vec.push(Token::new(
                    TokenType::StringLiteral,
                    text,
                    lexer.line,
                    lexer.column,
                ));
                lexer.advance(); // consume other "
            },
            '{' => token_vec.push(Token::new(
                TokenType::BraceLeft,
                "{",
                lexer.line,
                lexer.column
            )),
            '}' => token_vec.push(Token::new(
                TokenType::BraceRight,
                "}",
                lexer.line,
                lexer.column,
            )),
            '[' => token_vec.push(Token::new(
                TokenType::BracketLeft,
                "[",
                lexer.line,
                lexer.column,
            )),
            ']' => token_vec.push(Token::new(
                TokenType::BracketRight,
                "]",
                lexer.line,
                lexer.column,
            )),
            '(' => token_vec.push(Token::new(
                TokenType::ParenLeft,
                "(",
                lexer.line,
                lexer.column,
            )),
            ')' => token_vec.push(Token::new(
                TokenType::ParenRight,
                ")",
                lexer.line,
                lexer.column,
            )),
            ',' => token_vec.push(Token::new(
                TokenType::Comma,
                ",",
                lexer.line,
                lexer.column,
            )),
            '.' => token_vec.push(Token::new(
                TokenType::Dot,
                ".",
                lexer.line,
                lexer.column,
            )),
            _ => { continue; }
        }
    };
    token_vec
}