use phf::phf_map; // it says theres an error here (theres not)

#[derive(Debug, Clone, PartialEq)]
pub enum TokenData {
    // =
    Equal,
    // ==
    Equivalent,
    // +
    Add,
    // +=
    AddAssign,
    // -
    Sub,
    // -=
    SubAssign,
    // *
    Mul,
    // *=
    MulAssign,
    // /
    Div,
    // /=
    DivAssign,
    // !=
    NotEqual,
    // <
    LessThan,
    // <=
    LessThanEqual,
    // >
    GreaterThan,
    // >=
    GreaterThanEqual,
    // ??
    Or,
    // &&
    And,
    // {
    OpenBody,
    // }
    CloseBody, 
    // [
    BracketLeft,
    // ]
    BracketRight,
    // (
    OpenParen,
    // )
    CloseParen, 
    // ;
    Semicolon,
    // ,
    Comma,
    // :
    Colon,
    // .
    Dot,
    // ..
    Range,
    // ->
    Arrow,
    // true or false
    BooleanLiteral(bool),
    // "<string>"
    StringLiteral(String),
    // '<char>'
    CharacterLiteral(char),
    // <0-9>..
    IntegerLiteral(i64),
    // <0.00-9.99>..
    FloatLiteral(f64),
    // any word
    Literal(String),
    // nothing
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub(crate) token_data: TokenData,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
impl<'a> Token {
    pub fn new(token_data: TokenData, line: usize, column: usize) -> Self { // we dont need value anymore
        Self { token_data, line, column, }
    }
    pub(crate) fn is_atomic(&self) -> bool {
        matches!( self.token_data,
              TokenData::Literal(_)
            | TokenData::IntegerLiteral(_)
            | TokenData::FloatLiteral(_)
            | TokenData::StringLiteral(_)
            | TokenData::BooleanLiteral(_)
        )
    }
}
pub static SINGLES: phf::Map<&'static str, TokenData> = phf_map! {
    "+" => TokenData::Add,
    "-" => TokenData::Sub,
    "/" => TokenData::Div,
    "*" => TokenData::Mul,
    "=" => TokenData::Equal,
    "<" => TokenData::LessThan,
    ">" => TokenData::GreaterThan,
    "." => TokenData::Dot,
    ";" => TokenData::Semicolon,
    ":" => TokenData::Colon,
    "," => TokenData::Comma,
    "{" => TokenData::OpenBody,
    "}" => TokenData::CloseBody,
    "[" => TokenData::BracketRight,
    "]" => TokenData::BracketLeft,
    "(" => TokenData::OpenParen,
    ")" => TokenData::CloseParen,
};

pub static DOUBLES: phf::Map<&'static str, TokenData> = phf_map! {
    "+=" => TokenData::AddAssign,
    "-=" => TokenData::SubAssign,
    "/=" => TokenData::DivAssign,
    "*=" => TokenData::MulAssign,
    "!=" => TokenData::NotEqual,
    "==" => TokenData::Equivalent,
    "<=" => TokenData::LessThanEqual,
    ">=" => TokenData::GreaterThanEqual,
    "??" => TokenData::Or,
    "&&" => TokenData::And,
    "->" => TokenData::Arrow,
    ".." => TokenData::Range,
};