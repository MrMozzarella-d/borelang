use std::env::args_os;
use std::fs::OpenOptions;
use std::io::Read;
use std::time;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod parser;
mod node;
mod ast_printer;
mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hello!");
    println!("taking time..");
    let time_start = time::SystemTime::now();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("program.br")?;
    println!("opened file");
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("read file content");
    println!("starting lexer");
    let mut lexer = Lexer::new(&*content);
    let tokens = lexer.tokenize();
    println!("lexer finished tokenizing:");
    for token in tokens.iter() {
        println!("      {:?}", token);
    }
    println!("starting parser");
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();
    println!("parser finished parsing");

    let time_end = time::SystemTime::now();
    let time = time_end.duration_since(time_start)?.as_secs_f64();
    println!("took {} seconds!", time);

    let mut printer = ast_printer::AstTreePrinter::new();
    printer.print_program(&statements);

    Ok(())
}
