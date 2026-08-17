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
    // ;;
    Return,
    // ,
    Comma,
    // :
    Colon,
    // .
    Dot,
    // ..
    Range,
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

#[derive(Debug, Clone)]
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

pub static OPERATORS: phf::Map<&'static str, TokenData> = phf_map! {
    "+" => TokenData::Add,
    "+=" => TokenData::AddAssign,
    "-" => TokenData::Sub,
    "-=" => TokenData::SubAssign,
    "/" => TokenData::Div,
    "/=" => TokenData::DivAssign,
    "*" => TokenData::Mul,
    "*=" => TokenData::MulAssign,
    "=" => TokenData::Equal,
    "!=" => TokenData::NotEqual,
    "==" => TokenData::Equivalent,
    "<" => TokenData::LessThan,
    "<=" => TokenData::LessThanEqual,
    ">" => TokenData::GreaterThan,
    ">=" => TokenData::GreaterThanEqual,
};