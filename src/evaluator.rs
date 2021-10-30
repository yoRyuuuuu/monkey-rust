use crate::ast::{Expression, Program, Statement};
use crate::object::Object;
use anyhow::Result;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        let evaluator = Self {};
        evaluator
    }

    pub fn evaluate(&mut self, program: Program) -> Result<Object> {
        let mut result = Object::Null;
        for stmt in program.statements {
            result = self.evaluate_statement(stmt)?;
        }

        Ok(result)
    }

    pub fn evaluate_statement(&mut self, stmt: Statement) -> Result<Object> {
        match stmt {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::Let { ident, value } => todo!(),
            Statement::Return(_) => todo!(),
        }
    }

    pub fn evaluate_expression(&mut self, expr: Expression) -> Result<Object> {
        match expr {
            Expression::Int(value) => Ok(Object::Int(value)),
            Expression::Ident(_) => todo!(),
            Expression::Boolean(value) => Ok(Object::Boolean(value)),
            Expression::Prefix { op, right } => {
                let right = self.evaluate_expression(*right)?;
                self.evaluate_prefix_expression(op, right)
            }
            Expression::Infix { left, op, right } => todo!(),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => todo!(),
            Expression::Function { parameters, body } => todo!(),
            Expression::Call {
                function,
                arguments,
            } => todo!(),
        }
    }

    fn evaluate_prefix_expression(&mut self, op: String, right: Object) -> Result<Object> {
        match op.as_str() {
            "!" => self.evaluate_bang_operator_expression(right),
            _ => Ok(Object::Null),
        }
    }

    fn evaluate_bang_operator_expression(&mut self, right: Object) -> Result<Object> {
        match right {
            Object::Boolean(true) => Ok(Object::Boolean(false)),
            Object::Boolean(false) => Ok(Object::Boolean(true)),
            Object::Null => Ok(Object::Boolean(true)),
            _ => Ok(Object::Boolean(false)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser};

    #[test]
    fn test_eval_interger_expression() {
        let tests = vec![("5", 5), ("10", 10)];
        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Int(test.1));
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Boolean(test.1));
        }
    }

    fn test_evaluate(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut evaluator = Evaluator::new();
        evaluator.evaluate(program).unwrap()
    }
}
