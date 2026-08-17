extern crate core;

use std::fs::OpenOptions;
use std::io::{Read};

use crate::lexer::Lexer;
use crate::parser::Parser;

mod token;
mod lexer;
mod parser;
mod node;
mod ast_printer;
mod error;
mod interpreter;
mod stdlib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("hello!");
    let time = std::time::SystemTime::now();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("program.br")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut lexer = Lexer::new(&*content);
    let tokens = lexer.tokenize();
    // for token in tokens.iter() {
    //     println!("      {:?}", token);
    // }
    let mut parser = Parser::new(tokens);
    let statements_res = parser.parse();
    if statements_res.is_err() {
        eprintln!("{}", statements_res.unwrap_err());
    } else {
        let map = statements_res.map(|v| v).unwrap();
        let interpreter = interpreter::Interpreter::new(map);
        if let Err(e) = interpreter.run() {
            eprintln!("{}", e);
        }
    }
    let time_now = std::time::SystemTime::now();
    let dur = time_now.duration_since(time)?.as_nanos();
    println!("took: {}", dur);

    Ok(())
}
