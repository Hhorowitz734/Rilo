// src/main.rs

use std::env;
use std::fs;
use std::process;

mod token;
mod lexer;
mod parser;
mod ast;

use lexer::Lexer;
use token::Token;
use ast::Function;
use parser::Parser;

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
    
    // Loop over and lex source file
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
   
        let tok = lexer.next_token();
        tokens.push(tok.clone());

        if tok == Token::Eof {
            break;
        }

    }

    let mut parser = Parser::new(tokens);
    let functions = parser.parse();

    for func in functions {
        println!("{:#?}", func);
    }
}
