use crate::token;
use crate::token::{Token, TokenType};

pub struct Lexer {
    source: String,
    //start: usize,
    current: usize,
    line: usize,
    column: usize,
}
impl Lexer {
    pub fn new(source:String) -> Self {
        Self {
            source,
            //start: 0,
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

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut token_vec = Vec::new();
        while self.current < self.source.len() {
            let char = self.advance();
            if char.is_alphabetic() || char == '_' {
                let start_pos = self.current - char.len_utf8();
                while self.current < self.source.len() {
                    let next_char = self.peek();
                    if next_char.is_alphanumeric() || next_char == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let end_pos = self.current;
                let identifier = &self.source[start_pos..end_pos];
                if token::KEYWORDS.contains_key(identifier) {
                    token_vec.push(Token::new(
                        *token::KEYWORDS.get(identifier).unwrap(),
                        identifier,
                        self.line,
                        self.column,
                    ));
                } else {
                    token_vec.push(Token::new(
                        TokenType::Identifier,
                        identifier,
                        self.line,
                        self.column,
                    ));
                }
                continue;
            }
            if char.is_numeric() {
                let start_pos = self.current - char.len_utf8();
                while self.current < self.source.len() {
                    let next = self.peek();
                    if next.is_numeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let end_pos = self.current;
                let val = &self.source[start_pos..end_pos];
                token_vec.push(Token::new(
                    TokenType::Number,
                    val,
                    self.line,
                    self.column,
                ))
            }

            match char {
                ' ' | '\n' |  '\r' |  '\t'  => { continue; },
                '@'                     => {
                    let peek_next = self.peek();
                    if peek_next == '[' { // alias reference
                        let _ = self.advance();
                        let start_pos = self.current - char.len_utf8();
                        while self.current < self.source.len() {
                            let next_char = self.peek();
                            if next_char.is_alphanumeric() || next_char == '_' {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        let end_pos = self.current;
                        _ = self.advance(); // consume ]
                        let val = &self.source[start_pos..end_pos];
                        token_vec.push(Token::new(
                            TokenType::StackAliasReference,
                            val,
                            self.line,
                            self.column,
                        ))
                    } else if peek_next.is_numeric() {
                        let start_pos = self.current;
                        while self.current < self.source.len() {
                            let char = self.peek();
                            if !char.is_numeric() {
                                break;
                            } else {
                                self.advance();
                            }
                        }
                        let end_pos = self.current;
                        let val = &self.source[start_pos..end_pos];
                        token_vec.push(Token::new(
                            TokenType::StackPointReference,
                            val,
                            self.line,
                            self.column,
                        ))
                    } else {
                        eprintln!("Syntax Error: Unrecognized character for stack point reference '{}'", char);
                    }
                },
                '+' | '/' | '*' | '=' | '-' => {
                    match self.peek_next() {
                        '=' => {
                            let comb_string = format!("{char}=");
                            let comb = comb_string.as_str();
                            token_vec.push(Token::new(
                                *token::OPERATORS.get(comb).unwrap(),
                                comb,
                                self.line,
                                self.column,
                            ))
                        }
                        _ => {
                            let string = char.to_string();
                            let str = string.as_str();
                            token_vec.push(Token::new(
                                *token::OPERATORS.get(str).unwrap(),
                                str,
                                self.line,
                                self.column,
                            ))
                        }
                    }
                },
                '"'                 => {
                    let start_pos = self.current;
                    while self.current < self.source.len() {
                        let next = self.peek();
                        if next == '"' {
                            break;
                        }
                        self.advance();
                    }
                    let end_pos = self.current;
                    let text = &self.source[start_pos..end_pos];
                    token_vec.push(Token::new(
                        TokenType::StringLiteral,
                        text,
                        self.line,
                        self.column,
                    ));
                    self.advance(); // consume other "
                },
                '{' => token_vec.push(Token::new(
                    TokenType::BraceLeft,
                    "{",
                    self.line,
                    self.column,
                )),
                '}' => token_vec.push(Token::new(
                    TokenType::BraceRight,
                    "}",
                    self.line,
                    self.column,
                )),
                '[' => token_vec.push(Token::new(
                    TokenType::BracketLeft,
                    "[",
                    self.line,
                    self.column,
                )),
                ']' => token_vec.push(Token::new(
                    TokenType::BracketRight,
                    "]",
                    self.line,
                    self.column,
                )),
                '(' => token_vec.push(Token::new(
                    TokenType::ParenLeft,
                    "(",
                    self.line,
                    self.column,
                )),
                ')' => token_vec.push(Token::new(
                    TokenType::ParenRight,
                    ")",
                    self.line,
                    self.column,
                )),
                ',' => token_vec.push(Token::new(
                    TokenType::Comma,
                    ",",
                    self.line,
                    self.column,
                )),
                '.' => token_vec.push(Token::new(
                    TokenType::Dot,
                    ".",
                    self.line,
                    self.column,
                )),
                _ => { continue; }
            }
        };
        token_vec.push(Token::new( // push EOF after done
            TokenType::EOF,
            "",
            self.line,
            self.column,
        ));
        token_vec
    }
}
