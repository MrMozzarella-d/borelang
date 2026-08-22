#![feature(duration_millis_float)]
use std::fs::OpenOptions;
use std::io::Read;

use borelang::runtime::interpreter::Interpreter;
use borelang::syntax::{lexer::Lexer, parser::Parser};

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
    let dur_lex = std::time::SystemTime::now().duration_since(time_lexer)?.as_millis_f64();

    let time_parser = std::time::SystemTime::now();
    let mut parser = Parser::new(tokens);
    let statements_res = parser.parse();
    let dur_parse = std::time::SystemTime::now().duration_since(time_parser)?.as_millis_f64();

    let mut dur_inter = None;
    if statements_res.is_err() {
        eprintln!("{}", statements_res.unwrap_err());
    } else {
        let time_interpret = std::time::SystemTime::now();
        let map = statements_res.map(|v| v).unwrap();
        
        let mut interpreter = Interpreter::new(map);
        if let Err(e) = interpreter.run() {
            eprintln!("{}", e);
        }
        dur_inter = Some(std::time::SystemTime::now().duration_since(time_interpret)?.as_millis_f64());
    }

    let time_now = std::time::SystemTime::now();
    let dur = time_now.duration_since(time_start)?.as_millis_f64();
    println!();
    println!("total: {}", dur);
    println!("time lexer: {}", dur_lex);
    println!("time parser: {}", dur_parse);
    if let Some(dur_inter) = dur_inter {
        println!("time interpreter: {}", dur_inter);
    } else {
        println!("interpreter crashed")
    }
    Ok(())
}
