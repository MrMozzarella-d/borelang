extern crate core;

use std::fs::OpenOptions;
use std::io::Read;
use crate::ast_printer::AstTreePrinter;
//use crate::ast_printer::AstTreePrinter;
use crate::lexer::Lexer;
use crate::parser::Parser;

mod ast_printer;
mod error;
mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;

mod builtins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("hello!");
    let time_start = std::time::SystemTime::now();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("program.br")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let time_lexer = std::time::SystemTime::now();
    let mut lexer = Lexer::new(&*content);
    let tokens = lexer.tokenize();
    let dur_lex = std::time::SystemTime::now().duration_since(time_lexer)?.as_nanos();
    // for token in tokens.iter() {
    //     println!("      {:?}", token);
    // }
    let time_parser = std::time::SystemTime::now();
    let mut parser = Parser::new(tokens);
    let dur_parse = std::time::SystemTime::now().duration_since(time_parser)?.as_nanos();
    let statements_res = parser.parse();
    let mut dur_inter = None;
    if statements_res.is_err() {
        eprintln!("{}", statements_res.unwrap_err());
    } else {
        // let mut ast_printer = AstTreePrinter::new();
        // ast_printer.print_program(&*statements_res.unwrap());
        let time_interpret = std::time::SystemTime::now();
        let map = statements_res.map(|v| v).unwrap();
        let interpreter = interpreter::Interpreter::new(map);
        if let Err(e) = interpreter.run() {
            eprintln!("{}", e);
        }
        dur_inter = Some(std::time::SystemTime::now().duration_since(time_interpret)?.as_nanos());
    }

    let time_now = std::time::SystemTime::now();
    let dur = time_now.duration_since(time_start)?.as_nanos();
    println!("total: {}", dur);
    println!("time lexer: {}", dur_lex);
    println!("time parser: {}", dur_parse);
    if let Some(dur_inter) = dur_inter {
        println!("time interpreter: {}", dur_inter);
    }
    Ok(())
}
