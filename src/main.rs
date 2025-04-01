// src/main.rs

use std::env;
use std::fs;
use std::process;

mod token;
mod lexer;

use lexer::Lexer;
use token::Token;

fn main() {
    
    // Get args
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: rilo <file>");
        process::exit(1);
    }
    
    let filename = &args[1];

    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", filename, err);
        process::exit(1);
    });
    
    // Loop over and parse source file
    let mut lexer = Lexer::new(&source);
    loop {
   
        let tok = lexer.next_token();
        println!("{:?}", tok);

        if tok == Token::Eof {
            break;
        }

    }
}
