#![allow(dead_code)] // change later, just annoying rn
#![deny(rust_2018_idioms)]

use interpreter::Compile;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::io::{self, Write};

mod ast;
mod interpreter;
mod parser;
mod scanner;
mod tokens;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if !args[1].trim().is_empty() {
        match interpreter::Interpreter::from_file(&args[1]) {
            Ok(result) => {
                println!("Result: {}", result);
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    } else {
        repl();
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
    let mut scanner = Scanner::new(input);
    let mut parser: Parser;
    match scanner.scan_tokens() {
        Ok(tokens) => {
            println!("TOKENS: {:?}", tokens);
            parser = Parser::new(tokens);
        }
        Err(err) => {
            println!("Error occurred when scanning input string: {}", err);
            return;
        }
    }
    let ast = parser.parse();
    match ast {
        Ok(ast) => {
            println!("Result: {}", Interpreter::from_ast(ast));
        }
        Err(str) => {
            println!("Error occurred when parsing: {}", str);
        }
    }
}
