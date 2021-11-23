use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::object::{self, Object};
use anyhow::Result;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        let evaluator = Self {};
        evaluator
    }

    pub fn evaluate(&mut self, program: Program) -> Result<Object> {
        let mut obj = Object::Null;
        for stmt in program.statements {
            obj = self.evaluate_statement(stmt)?;
            match obj {
                Object::Return(value) => return Ok(*value),
                Object::Error(_) => return Ok(obj),
                _ => (),
            }
        }

        Ok(obj)
    }

    fn evaluate_block_statement(&mut self, block: BlockStatement) -> Result<Object> {
        let mut obj = Object::Null;
        for stmt in block.statements {
            obj = self.evaluate_statement(stmt)?;
            match obj {
                Object::Return(value) => return Ok(*value),
                Object::Error(_) => return Ok(obj),
                _ => (),
            }
        }
        Ok(obj)
    }

    pub fn evaluate_statement(&mut self, stmt: Statement) -> Result<Object> {
        match stmt {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::Let { ident, value } => todo!(),
            Statement::Return(expr) => self.evaluate_return_statement(expr),
        }
    }

    fn evaluate_return_statement(&mut self, expr: Expression) -> Result<Object> {
        let obj = self.evaluate_expression(expr)?;
        if let Object::Error(_) = obj {
            return Ok(obj);
        }
        Ok(Object::Return(Box::new(obj)))
    }

    pub fn evaluate_expression(&mut self, expr: Expression) -> Result<Object> {
        match expr {
            Expression::Int(value) => Ok(Object::Int(value)),
            Expression::Ident(_) => todo!(),
            Expression::Boolean(value) => Ok(Object::Boolean(value)),
            Expression::Prefix { op, right } => {
                let right = self.evaluate_expression(*right)?;
                if let Object::Error(_) = right {
                    return Ok(right);
                }
                Ok(self.evaluate_prefix_expression(op, right))
            }
            Expression::Infix { left, op, right } => {
                let left = self.evaluate_expression(*left)?;
                if let Object::Error(_) = left {
                    return Ok(left);
                }
                let right = self.evaluate_expression(*right)?;
                if let Object::Error(_) = right {
                    return Ok(right);
                }
                Ok(self.evaluate_infix_expression(op, left, right))
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.evaluate_if_expression(*condition, consequence, alternative),
            Expression::Function { parameters, body } => todo!(),
            Expression::Call {
                function,
                arguments,
            } => todo!(),
        }
    }

    fn evaluate_if_expression(
        &mut self,
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Result<Object> {
        let condition = self.evaluate_expression(condition)?;
        if let Object::Error(_) = condition {
            return Ok(condition);
        }
        if Self::is_truthy(condition) {
            return self.evaluate_block_statement(consequence);
        }

        if let Some(alternative) = alternative {
            return self.evaluate_block_statement(alternative);
        }

        Ok(Object::Null)
    }

    fn evaluate_prefix_expression(&mut self, op: String, right: Object) -> Object {
        match op.as_str() {
            "!" => self.evaluate_bang_operator_expression(right),
            "-" => self.evaluate_minus_prefix_operator_expression(right),
            _ => Object::Error(format!("unknown operator: {}{}", op, &right.type_info())),
        }
    }

    fn evaluate_infix_expression(&mut self, op: String, left: Object, right: Object) -> Object {
        match (op.as_str(), left, right) {
            (_, Object::Int(l), Object::Int(r)) => self.evaluate_int_infix_expression(op, l, r),
            ("==", Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l == r),
            ("!=", Object::Boolean(l), Object::Boolean(r)) => Object::Boolean(l != r),
            (_, _left, _right) if _left.type_info() != _right.type_info() => {
                Object::Error(format!(
                    "type mismatch: {} {} {}",
                    _left.type_info(),
                    op,
                    _right.type_info()
                ))
            }
            (_, _left, _right) => Object::Error(format!(
                "unknown operator: {} {} {}",
                _left.type_info(),
                op,
                _right.type_info()
            )),
        }
    }

    fn evaluate_int_infix_expression(&mut self, op: String, left: i64, right: i64) -> Object {
        match op.as_str() {
            "+" => Object::Int(left + right),
            "-" => Object::Int(left - right),
            "*" => Object::Int(left * right),
            "/" => Object::Int(left / right),
            "<" => Object::Boolean(left < right),
            ">" => Object::Boolean(left > right),
            "==" => Object::Boolean(left == right),
            "!=" => Object::Boolean(left != right),
            _ => Object::Error(format!("unknown operator: INTEGER {} INTEGER", op)),
        }
    }

    fn evaluate_minus_prefix_operator_expression(&mut self, right: Object) -> Object {
        match right {
            Object::Int(value) => Object::Int(-value),
            _ => Object::Error(format!("unknown operator: -{}", right.type_info())),
        }
    }

    fn evaluate_bang_operator_expression(&mut self, right: Object) -> Object {
        match right {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        }
    }

    fn is_truthy(obj: Object) -> bool {
        match obj {
            Object::Null => false,
            Object::Boolean(value) => value,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser};

    #[test]
    fn test_evaluate_interger_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Int(test.1));
        }
    }

    #[test]
    fn test_evaluate_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Boolean(test.1));
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ("if (true) { 10 }", Object::Int(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1) { 10 }", Object::Int(10)),
            ("if (1 < 2) { 10 }", Object::Int(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Int(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Int(10)),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, test.1);
        }
    }

    #[test]
    fn test_return_statement() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                r#"if (10 > 1) {
if (10 > 1) {
    return 10;
    }
}"#,
                10,
            ),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Int(test.1));
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false;  5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                r#"if (10 > 1) {
    if (10 > 1) {
        return true + false;
    }
}"#,
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object.to_string(), format!("Error: {}", test.1))
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
