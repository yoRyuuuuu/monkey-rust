use std::fmt;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in self.statements.iter() {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let {
        ident: Expression,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { ident, value } => write!(f, "let {} = {};", ident, value),
            Statement::Return(expr) => write!(f, "return {};", expr),
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Ident(String),
    Int(i64),
    Boolean(bool),
    Prefix {
        op: String,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        op: String,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    Function {
        parameters: Vec<String>,
        body: BlockStatement,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Ident(value) => write!(f, "{}", value),
            Expression::Int(value) => write!(f, "{}", value),
            Expression::Boolean(value) => write!(f, "{}", value),
            Expression::Prefix { op, right } => write!(f, "({}{})", op, right),
            Expression::Infix { left, op, right } => write!(f, "({} {} {})", left, op, right),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {}", condition, consequence)?;
                match alternative {
                    Some(statements) => write!(f, "{}", statements)?,
                    None => (),
                };

                Ok(())
            }
            Expression::Function { parameters, body } => {
                let params = parameters.clone().join(", ");

                write!(f, "fn ({}) {{ {} }}", params, body)?;
                Ok(())
            }
            Expression::Call {
                function,
                arguments,
            } => {
                let mut args = vec![];
                for a in arguments {
                    args.push(a.to_string());
                }
                let args = args.join(", ");

                write!(f, "{}({})", function.to_string(), args)?;

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in self.statements.iter() {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
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
