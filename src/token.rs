use phf::phf_map;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Int,
    Float,
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
    KwWhile,
    KwFor,
    KwLet,
    KwFunction,
    KwMut,
    KwExtern,
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
#[derive(Debug, Copy, Clone)]
pub struct  Token<'a> {
    pub(crate) token_type: TokenType,
    pub(crate) value: &'a str,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, value: &'a str, line: usize, column: usize) -> Self {
        Self { token_type, value, line, column, }
    }
    pub(crate) fn is_operator(&self) -> bool {
        OPERATORS.values().any(|&x| x == self.token_type)
    }
    pub(crate) fn is_atomic(&self) -> bool {
        matches!( self.token_type,
              TokenType::Identifier
            | TokenType::Number
            | TokenType::StringLiteral
        )
    }
}

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "if" => TokenType::KwIf,
    "else" => TokenType::KwElse,
    "return" => TokenType::KwReturn,
    "as" => TokenType::KwAs,
    "break" => TokenType::KwBreak,
    "ext" => TokenType::KwExtern,
    "fn" => TokenType::KwFunction,
    "let" => TokenType::KwLet,
    "mut" => TokenType::KwMut,
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