use std::fs::OpenOptions;
use std::io::Read;

mod token;
mod lexer;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("program.br")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let tokens = lexer::tokenize(content);
    
    println!("Tokens:");
    for token in tokens {
        println!("{:?}", token)
    }
    
    Ok(())
}
