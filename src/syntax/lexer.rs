use crate::syntax::token;
use token::{Token, TokenData};

pub struct Lexer<'a> {
    source: &'a str,
    //start: usize,
    current: usize,
    line: usize,
    column: usize,
}
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
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
        let char = self.source.as_bytes()[self.current] as char;
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
        self.source.as_bytes()[self.current] as char
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return char::from(0); }
        self.source.as_bytes()[self.current + 1] as char
    }
    fn is_at_end(&self) -> bool { self.current >= self.source.len() }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut token_vec: Vec<Token> = Vec::new();
        while self.current < self.source.len() {
            let start_pos = self.current;
            let char = self.advance();
            if char.is_alphabetic() || char == '_' {
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
                if identifier == "false" || identifier == "true" {
                    token_vec.push(Token::new(
                        TokenData::BooleanLiteral(identifier == "true"),
                        self.line,
                        self.column,
                    ));
                    continue
                }
                token_vec.push(Token::new(
                    TokenData::Literal(identifier.to_string()),
                    self.line,
                    self.column,
                ));
                continue;
            }
            
            if char.is_numeric() {
                while self.current < self.source.len() {
                    let next = self.peek();
                    if next.is_numeric() || (next == '.' && self.peek_next().is_numeric()) || next == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let end_pos = self.current;
                let val = &self.source[start_pos..end_pos];
                let val = val.replace("_", "");
                if val.contains('.') {
                    token_vec.push(Token::new(
                        TokenData::FloatLiteral(val.parse().unwrap()),
                        self.line,
                        self.column,
                    ));
                } else { 
                    token_vec.push(Token::new(
                            TokenData::IntegerLiteral(val.parse().unwrap()),
                            self.line,
                            self.column, 
                        )); 
                }
                continue;
            }

            let s = format!("{}{}", char, self.peek());
            let single = format!("{}", char);
            let mut data = None;
            if token::DOUBLES.contains_key(s.as_str()) {
                data = Some(token::DOUBLES.get(s.as_str()).unwrap());
                self.advance(); 
            } else if token::SINGLES.contains_key(single.as_str()) {
                data = Some(token::SINGLES.get(single.as_str()).unwrap());
            }
            if let Some(d) = data {
                token_vec.push(Token::new(
                    d.clone(),
                    self.line,
                    self.column,
                ))   
            }
            match char {
                ' ' | '\n' |  '\r' |  '\t'  => { continue; },
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
                    self.advance(); // consume other "
                    token_vec.push(Token::new(
                        TokenData::StringLiteral(text.to_string()),
                        self.line,
                        self.column,
                    ));
                },
                '#' => {
                    while self.current < self.source.len() {
                        if self.peek() == '\n' {
                            break
                        } self.advance();
                    }
                }
                _ => { continue; }
            }
        };
        token_vec.push(Token::new( // push EOF after done
            TokenData::EOF,
            self.line,
            self.column,
        ));
        token_vec
    }
}
