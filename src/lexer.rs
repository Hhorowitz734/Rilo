// src/lexer.rs

use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    
    // Lexer constructor
    pub fn new(source: &'a str) -> Self {
        Self {
            input: source.chars().peekable(),
        }
    }
    
    // Parse next token
    pub fn next_token(&mut self) -> Token {
        
        self.skip_whitespace();
    
        match self.input.peek() {
            Some(&ch) => {
                match ch {
                    '+' => { self.input.next(); Token::Plus }

                    // Account for '-' vs '->' 
                    '-' => {
                        self.input.next();
                        if self.input.peek() == Some(&'>') {
                            self.input.next();
                            Token::Arrow
                        } else {
                            Token::Minus
                        }
                    }

                    '*' => { self.input.next(); Token::Star }
                    '/' => { self.input.next(); Token::Slash }
                    '=' => { self.input.next(); Token::Equal }
                    ':' => { self.input.next(); Token::Colon }
                    ';' => { self.input.next(); Token::Semicolon }
                    '(' => { self.input.next(); Token::LParen }
                    ')' => { self.input.next(); Token::RParen }
                    '{' => { self.input.next(); Token::LBrace }
                    '}' => { self.input.next(); Token::RBrace }

                    ch if ch.is_ascii_digit() => self.read_number(),
                    ch if ch.is_ascii_alphabetic() || ch == '_' => self.read_ident(),

                    // skip unknown chars for now
                    _ => {
                        self.input.next();
                        self.next_token()
                    }
                }
            }
            None => Token::Eof,
        }
    }

    // Skip ' '
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    // For ints
    fn read_number(&mut self) -> Token {
        let mut num = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        Token::Int(num.parse().unwrap())
    }

    // Read keywords
    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            "mut" => Token::Mut,
            _ => Token::Ident(ident),
        }
    }




}
