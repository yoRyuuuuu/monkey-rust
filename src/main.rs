use crate::parser::Parser;
use crate::{evaluator::Evaluator, lexer::Lexer};

mod ast;
mod errors;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod token;

use std::io::{self, Write};

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);
        let mut evaluator = Evaluator::new();
        match parser.parse_program() {
            Ok(program) => match evaluator.evaluate_program(program) {
                Ok(object) => println!("{}", object),
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
