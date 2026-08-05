use phf::phf_map;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifier,                
    StackPointReference,       
    StackAliasReference,        
    Number,                     
    StringLiteral,              
    // operators                //all done
    Equal,
    CompEqual,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Multiply,
    MultiplyEqual,
    Divide,
    DivideEqual,
    // keywords //all done
    KwIf,
    KwElse,
    KwReturn,
    KwBreak,
    KwProc,
    KwAlias,
    KwAs,
    // punctuation //all done
    BraceLeft,      // {
    BraceRight,     // }
    BracketLeft,    // [
    BracketRight,   // ]
    ParenLeft,      // (
    ParenRight,     // )
    Comma,
    Colon,
    Dot,

    EOF,
}
#[derive(Debug)]
pub struct  Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: String,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
impl Token {
    pub fn new(token_type: TokenType, value: &str, line: usize, column: usize) -> Self {
        Self { token_type, value: value.parse().unwrap(), line, column, }
    }
}

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "if" => TokenType::KwIf,
    "else" => TokenType::KwElse,
    "return" => TokenType::KwReturn,
    "alias" => TokenType::KwAlias,
    "as" => TokenType::KwAs,
    "proc" => TokenType::KwProc,
    "break" => TokenType::KwBreak,
};

pub static OPERATORS: phf::Map<&'static str, TokenType> = phf_map! {
    "+" => TokenType::Plus,
    "+=" => TokenType::PlusEqual,
    "-" => TokenType::Minus,
    "-=" => TokenType::MinusEqual,
    "/" => TokenType::Divide,
    "/=" => TokenType::DivideEqual,
    "*" => TokenType::Multiply,
    "*=" => TokenType::MultiplyEqual,
    "=" => TokenType::Equal,
    "==" => TokenType::CompEqual,
};