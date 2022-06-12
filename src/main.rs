#![deny(rust_2018_idioms)]

use interpreter::Compile;
use std::io::{self, Write};

mod ast;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap(); // ignore args[0]
    
    match args.next() {
        Some(file) => {
            if file.trim().is_empty() {
                match interpreter::Interpreter::from_file(&file) {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => println!("Error: {}", err),
                }
            }
        }
        
        None => repl(),
    }
}

fn repl() {
    let mut buffer = String::new();
    
    loop {
        print!(">>> ");
        
        io::stdout().flush().expect("Could not flush stdout.");
        
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input.");
        
        let trimmed = buffer.trim();
        
        if trimmed.is_empty() {
            continue;
        } else {
            eval_input(trimmed);
        }
    }
}

fn eval_input(input: &str) {
    match interpreter::Interpreter::from_input(input) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        todo!();
    }
}