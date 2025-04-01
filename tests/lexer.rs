// tests/lexer.rs

use Rilo::lexer::Lexer;
use Rilo::token::Token;

/// Test simple 'let' statement
#[test]
fn test_let_statement() {
    let mut lexer = Lexer::new("let x = 42;");
    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::Int(42));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test function signature and symbols
#[test]
fn test_fn_signature() {
    let mut lexer = Lexer::new("fn main() -> int {}");
    assert_eq!(lexer.next_token(), Token::Fn);
    assert_eq!(lexer.next_token(), Token::Ident("main".to_string()));
    assert_eq!(lexer.next_token(), Token::LParen);
    assert_eq!(lexer.next_token(), Token::RParen);
    assert_eq!(lexer.next_token(), Token::Arrow);
    assert_eq!(lexer.next_token(), Token::Ident("int".to_string()));
    assert_eq!(lexer.next_token(), Token::LBrace);
    assert_eq!(lexer.next_token(), Token::RBrace);
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test math operators
#[test]
fn test_arithmetic_operators() {
    let mut lexer = Lexer::new("+ - * /");
    assert_eq!(lexer.next_token(), Token::Plus);
    assert_eq!(lexer.next_token(), Token::Minus);
    assert_eq!(lexer.next_token(), Token::Star);
    assert_eq!(lexer.next_token(), Token::Slash);
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test '->' vs '-' tokens
#[test]
fn test_arrow_and_minus() {
    let mut lexer = Lexer::new("-> -");
    assert_eq!(lexer.next_token(), Token::Arrow);
    assert_eq!(lexer.next_token(), Token::Minus);
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test that 'letx' is not a keyword
#[test]
fn test_keyword_prefix_identifier() {
    let mut lexer = Lexer::new("letx");
    assert_eq!(lexer.next_token(), Token::Ident("letx".to_string()));
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test identifier with underscore and numbers
#[test]
fn test_identifier_with_underscore() {
    let mut lexer = Lexer::new("foo_bar123");
    assert_eq!(lexer.next_token(), Token::Ident("foo_bar123".to_string()));
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test empty input
#[test]
fn test_empty_input() {
    let mut lexer = Lexer::new("");
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test code with newlines
#[test]
fn test_multiline_code() {
    let mut lexer = Lexer::new("let a = 1;\nreturn a;");
    assert_eq!(lexer.next_token(), Token::Let);
    assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Equal);
    assert_eq!(lexer.next_token(), Token::Int(1));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::Return);
    assert_eq!(lexer.next_token(), Token::Ident("a".to_string()));
    assert_eq!(lexer.next_token(), Token::Semicolon);
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test multiple integers in a row
#[test]
fn test_multiple_integers() {
    let mut lexer = Lexer::new("1 23 456");
    assert_eq!(lexer.next_token(), Token::Int(1));
    assert_eq!(lexer.next_token(), Token::Int(23));
    assert_eq!(lexer.next_token(), Token::Int(456));
    assert_eq!(lexer.next_token(), Token::Eof);
}

/// Test unknown characters are skipped
#[test]
fn test_unknown_characters() {
    let mut lexer = Lexer::new("@#$");
    assert_eq!(lexer.next_token(), Token::Eof);
}
