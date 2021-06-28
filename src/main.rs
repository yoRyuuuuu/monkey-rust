use crate::lexer::Lexer;
use crate::token::TokenKind;

mod ast;
mod errors;
mod lexer;
mod parser;
mod token;

use std::io;

fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    return stdout.flush();
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};

    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();
    let vec = vec![10, 20, 30];

    loop {
        prompt("> ").unwrap();
        if let Some(Ok(line)) = lines.next() {
            let mut lexer = Lexer::new(&line);
            while let tok = lexer.next_token() {
                match tok.kind {
                    TokenKind::Eof => break,
                    _ => println!("{:?}", tok),
                }
            }
        } else {
            break;
        }
    }
}
