#![deny(rust_2018_idioms)]

use interpreter::Compile;
use std::io::{self, Write};

mod ast;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        repl();
    } else if !args[1].trim().is_empty() {
        match interpreter::Interpreter::from_file(&args[1]) {
            Ok(result) => {
                println!("Result: {}", result);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
}

fn repl() {
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Could not flush stdout.");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .ok()
            .expect("Failed to read input.");
        if buffer.trim().is_empty() {
            continue;
        } else {
            eval_input(buffer.trim());
        }
    }
}

fn eval_input(input: &str) {
    match interpreter::Interpreter::from_input(input) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err)
    }
}
