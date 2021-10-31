use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Int(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(obj) => write!(f, "{}", *obj),
        }
    }
}
