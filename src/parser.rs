// src/parser.rs

use crate::token::Token;
use crate::ast::{Expr, BinaryOp, Stmt, Function};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        // Look ahead without advancing

        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        // Advance to next token

        let tok = self.peek().clone();
        self.pos += 1;
        tok
    }
    
    fn expect(&mut self, expected: Token) {
        // Throws errors for unexpected tokens

        let tok = self.advance();
        if tok != expected {
            panic!("Expected {:?}, got {:?}", expected, tok);
        }
    }

    fn expect_ident(&mut self) -> String {
        // Consume and return identifier string 

        match self.advance() {
            Token::Ident(name) => name,
            other => panic!("Expected identifier, got {:?}", other),
        }
    }

    fn parse_expr(&mut self) -> Expr {
        // Parses expressions

        let mut left = self.parse_primary();

        while let Some(op) = self.match_binary_op() {
            let right = self.parse_primary();
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        left
    }
   
    
    fn parse_stmt(&mut self) -> Stmt {
        // Parses statements

        match self.peek() {
            Token::Let => {
                self.advance();
                let name = self.expect_ident();
                self.expect(Token::Equal);
                let value = self.parse_expr();
                self.expect(Token::Semicolon);
                Stmt::Let { name, value }
            }

            Token::Return => {
                self.advance();
                let value = self.parse_expr();
                self.expect(Token::Semicolon);
                Stmt::Return(value)
            }
            _ => panic!("Unexpected token in statement: {:?}", self.peek()),
        }
    }

    fn parse_primary(&mut self) -> Expr {
        match self.advance() {
            Token::Int(n) => Expr::Int(n),
            Token::Ident(name) => Expr::Var(name),
            other => panic!("Unexpected token in primary expression: {:?}", other),
        }
    }

    fn match_binary_op(&mut self) -> Option<BinaryOp> {
        match self.peek() {
            Token::Plus => {
                self.advance();
                Some(BinaryOp::Add)
            }
            Token::Minus => {
                self.advance();
                Some(BinaryOp::Sub)
            }
            Token::Star => {
                self.advance();
                Some(BinaryOp::Mul)
            }
            Token::Slash => {
                self.advance();
                Some(BinaryOp::Div)
            }
            _ => None,
        }
    }

    fn parse_function(&mut self) -> Function {
        // Parses functions

        // Parse function definition
        self.expect(Token::Fn);
        let name = self.expect_ident();
        self.expect(Token::LParen);
        self.expect(Token::RParen);
        self.expect(Token::Arrow);
        let _ret_type = self.expect_ident();
        self.expect(Token::LBrace);
        
        // Parse body
        let mut body = Vec::new();
        while self.peek() != &Token::RBrace {
            body.push(self.parse_stmt());
        }

        self.expect(Token::RBrace);  // Make sure function ends properly
    
        Function { name, body }
    }

    pub fn parse(&mut self) -> Vec<Function> {
        // Entry point for parser, parses a program into a list of functions
    
        let mut functions = Vec::new();
        
        while *self.peek() != Token::Eof {
            functions.push(self.parse_function());
        }

        functions
    }
}
