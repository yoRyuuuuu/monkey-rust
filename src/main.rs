use crate::lexer::Lexer;
use crate::parser::Parser;

mod ast;
mod errors;
mod lexer;
mod parser;
mod token;

use std::io::{self, Write};

fn main() {
    // let stdin = stdin();
    // let stdin = stdin.lock();
    // let stdin = BufReader::new(stdin);
    // let mut lines = stdin.lines();

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
