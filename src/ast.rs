use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement { ident: Expression, value: Expression },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Ident(String),
    Int(i64),
    Prefix {op: String, right: Box<Expression>}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

