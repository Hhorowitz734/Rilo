// tests/parser.rs

use rilo::lexer::Lexer;
use rilo::parser::Parser;
use rilo::token::Token;
use rilo::ast::{Expr, BinaryOp, Stmt, Function};


#[test]
fn test_empty_function() {
    let src = "fn main() -> int {}";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    assert_eq!(funcs.len(), 1);
    assert_eq!(funcs[0].name, "main");
    assert_eq!(funcs[0].body.len(), 0);
}

#[test]
fn test_let_stmt() {
    let src = "fn main() -> int { let x = 5; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Let { name, value } = &funcs[0].body[0] {
        assert_eq!(name, "x");
        assert!(matches!(value, Expr::Int(5)));
    } else {
        panic!("Expected let statement");
    }
}

#[test]
fn test_return_stmt() {
    let src = "fn main() -> int { return 1; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Return(expr) = &funcs[0].body[0] {
        assert!(matches!(expr, Expr::Int(1)));
    } else {
        panic!("Expected return statement");
    }
}

#[test]
fn test_return_binary_expr() {
    let src = "fn main() -> int { return x + 1; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Return(Expr::Binary { left, op, right }) = &funcs[0].body[0] {
        assert!(matches!(**left, Expr::Var(ref name) if name == "x"));
        assert_eq!(*op, BinaryOp::Add);
        assert!(matches!(**right, Expr::Int(1)));
    } else {
        panic!("Expected binary return");
    }
}

#[test]
fn test_multiple_stmts() {
    let src = "fn main() -> int { let x = 5; return x; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    assert_eq!(funcs[0].body.len(), 2);
}

#[test]
fn test_sub_expr() {
    let src = "fn main() -> int { return x - 2; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Return(Expr::Binary { op, .. }) = &funcs[0].body[0] {
        assert_eq!(*op, BinaryOp::Sub);
    } else {
        panic!("Expected subtraction");
    }
}

#[test]
fn test_nested_binary_expr() {
    let src = "fn main() -> int { return x + 2 * 3; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Return(Expr::Binary { op: BinaryOp::Add, .. }) = &funcs[0].body[0] {
        // not checking associativity yet — just validating structure
    } else {
        panic!("Expected binary add expr");
    }
}

#[test]
fn test_variable_expr() {
    let src = "fn main() -> int { return x; }";
    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    if let Stmt::Return(Expr::Var(name)) = &funcs[0].body[0] {
        assert_eq!(name, "x");
    } else {
        panic!("Expected variable return");
    }
}

#[test]
fn test_multiple_functions() {
    let src = r#"
        fn a() -> int { return 1; }
        fn b() -> int { return 2; }
    "#;

    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);
    let mut parser = Parser::new(tokens);
    let funcs = parser.parse();

    assert_eq!(funcs.len(), 2);
    assert_eq!(funcs[0].name, "a");
    assert_eq!(funcs[1].name, "b");
}

#[test]
fn test_parse_failure() {
    let src = "fn main() -> int { let = 5; }"; // invalid syntax (missing identifier)

    let mut lexer = Lexer::new(src);
    let tokens = collect_tokens(&mut lexer);

    let tokens = collect_tokens(&mut lexer);
    let result = std::panic::catch_unwind(move || {
        let mut parser = Parser::new(tokens);
        parser.parse();
    });
    assert!(result.is_err(), "Parser should panic on invalid syntax");
}


fn collect_tokens(lexer: &mut Lexer) -> Vec<Token> {
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if tok == Token::Eof {
            break;
        }
        tokens.push(tok);
    }
    tokens
}
