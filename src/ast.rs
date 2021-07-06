use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    LetStatement {
        ident: Expression,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Int(i64),
    Prefix {
        op: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    Lessgreater,
    Sum,
    Product,
    Prefix,
    Call,
}
