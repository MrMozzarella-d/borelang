use phf::phf_map;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TokenData<'a> {
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
    // true or false
    BooleanLiteral(bool),
    // "<string>"
    StringLiteral(&'a str),
    // '<char>'
    CharacterLiteral(char),
    // <0-9>..
    IntegerLiteral(i64),
    // <0.00-9.99>..
    FloatLiteral(f64),
    // any word
    Literal(&'a str),
    // nothing
    EOF,
}

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub(crate) token_data: TokenData<'a>,
    pub(crate) line: usize,
    pub(crate) column: usize,
}
impl<'a> Token<'a> {
    pub fn new(token_data: TokenData<'a>, line: usize, column: usize) -> Self { // we dont need value anymore
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