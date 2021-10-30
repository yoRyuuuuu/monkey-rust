use crate::lexer::Lexer;
use crate::parser::Parser;

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
        match parser.parse_program() {
            Ok(program) => print!("{}", program),
            Err(e) => eprintln!("{}", e),
        }
    }
}
