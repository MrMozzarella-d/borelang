use phf::phf_map;

pub enum TokenType {
    Identifier,
    StackPoint,
    Number,
    StringLiteral,
    // operators
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    // keywords
    KwIf,
    KwElse,
    KwReturn,
    KwBreak,
    KwProc,
    KwAlias,
    KwAs,
}
pub struct  Token {
    token_type: TokenType,
    value: String,
    line: u32,
    column: u32,
}
impl Token {
    pub fn new(token_type: TokenType, value: String, line: u32, column: u32) -> Self {
        Self { token_type, value, line, column, }
    }
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "if" => TokenType::KwIf,
    "else" => TokenType::KwElse,
    "return" => TokenType::KwReturn,
    "alias" => TokenType::KwAlias,
    "as" => TokenType::KwAs,
    "proc" => TokenType::KwProc,
    "break" => TokenType::KwBreak,
};