// src/token.rs

#[derive (Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    Return,
    Mut,

    // Identifiers
    Ident(String),
    Int(i32),

    // Symbols
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Colon,
    Arrow,
    Semicolon,

    // Brackets
    LParen,
    RParen,
    LBrace,
    RBrace,

    // End of input
    Eof,
}
