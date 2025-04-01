// src/ast.rs

#[derive(Debug)]
pub enum Expr {
    Int(i32),
    Var(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Return(Expr),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Vec<Stmt>,
}
