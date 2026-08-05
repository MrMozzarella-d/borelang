use std::fs::OpenOptions;
use std::io::Read;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod parser;
mod node;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("program.br")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize();
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    Ok(())
}
