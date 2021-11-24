use std::fmt;

use crate::{ast::BlockStatement, environment::Environment};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Function {
        parameters: Vec<String>,
        body: BlockStatement,
        environment: Environment,
    },
    Error(String),
}

impl Object {
    pub fn type_info(&self) -> String {
        match self {
            Object::Int(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::Error(_) => "FUNCTION",
            _ => unreachable!(),
        }
        .to_string()
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(obj) => write!(f, "{}", *obj),
            Object::Function {
                parameters,
                body,
                environment: _,
            } => {
                let params = parameters
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "fn ({}) {{ {} }}", params, body)
            }
            Object::Error(obj) => write!(f, "Error: {}", obj),
        }
    }
}
