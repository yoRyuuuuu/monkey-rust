use crate::ast::{BlockStatement, Expression, Program, Statement};
use crate::environment::Environment;
use crate::object::Object;

pub struct Evaluator<'a> {
    pub env: &'a mut Environment,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a mut Environment) -> Evaluator {
        Self { env }
    }

    pub fn evaluate(&mut self, program: Program) -> Object {
        let mut obj = Object::Null;
        for stmt in program.statements {
            obj = self.evaluate_statement(stmt);
            match obj {
                Object::Return(value) => return *value,
                Object::Error(_) => return obj,
                _ => (),
            }
        }
        obj
    }

    fn evaluate_block_statement(&mut self, block: BlockStatement) -> Object {
        let mut obj = Object::Null;
        for stmt in block.statements {
            obj = self.evaluate_statement(stmt);
            match obj {
                Object::Return(value) => return *value,
                Object::Error(_) => return obj,
                _ => (),
            }
        }
        obj
    }

    pub fn evaluate_statement(&mut self, stmt: Statement) -> Object {
        match stmt {
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::Let { ident, value } => self.evaluate_let_statement(ident, value),
            Statement::Return(expr) => self.evaluate_return_statement(expr),
        }
    }

    fn evaluate_let_statement(&mut self, ident: Expression, expr: Expression) -> Object {
        let name = match ident {
            Expression::Ident(name) => name,
            _ => unreachable!(),
        };
        let obj = self.evaluate_expression(expr);
        if let Object::Error(_) = obj {
            return obj;
        }
        self.env.set(&name, obj)
    }

    fn evaluate_return_statement(&mut self, expr: Expression) -> Object {
        let obj = self.evaluate_expression(expr);
        if let Object::Error(_) = obj {
            return obj;
        }
        Object::Return(Box::new(obj))
    }

    pub fn evaluate_expression(&mut self, expr: Expression) -> Object {
        match expr {
            Expression::Int(value) => Object::Int(value),
            Expression::Ident(name) => self.evaluate_identifier(name),
            Expression::Boolean(value) => Object::Boolean(value),
            Expression::Prefix { op, right } => {
                let right = self.evaluate_expression(*right);
                if let Object::Error(_) = right {
                    return right;
                }
                self.evaluate_prefix_expression(op, right)
            }
            Expression::Infix { left, op, right } => {
                let left = self.evaluate_expression(*left);
                if let Object::Error(_) = left {
                    return left;
                }
                let right = self.evaluate_expression(*right);
                if let Object::Error(_) = right {
                    return right;
                }
                self.evaluate_infix_expression(op, left, right)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.evaluate_if_expression(*condition, consequence, alternative),
            Expression::Function { parameters, body } => Object::Function {
                parameters,
                body,
                environment: self.env.clone(),
            },
            Expression::Call {
                function,
                arguments,
            } => {
                let func = self.evaluate_expression(*function);
                if let Object::Error(_) = func {
                    return func;
                }
                let args = self.evaluate_expressions(arguments);
                self.apply_function(func, args)
            }
        }
    }

    fn apply_function(&mut self, func: Object, args: Vec<Object>) -> Object {
        match func {
            Object::Function {
                parameters,
                body,
                environment,
            } => {
                let mut env = Environment::new_enclosed(environment);
                for (i, param) in parameters.iter().enumerate() {
                    env.set(param, args[i].clone());
                }
                let mut evaluator = Evaluator::new(&mut env);
                let obj = evaluator.evaluate_block_statement(body);
                match obj {
                    Object::Return(obj) => *obj,
                    _ => obj,
                }
            }
            _ => Object::Error(format!("not a function: {}", func.type_info())),
        }
    }

    fn evaluate_expressions(&mut self, exprs: Vec<Expression>) -> Vec<Object> {
        let mut result = vec![];
        for expr in exprs {
            let obj = self.evaluate_expression(expr);
            if let Object::Error(_) = obj {
                return vec![obj];
            }
            result.push(obj);
        }
        result
    }

    fn evaluate_if_expression(
        &mut self,
        condition: Expression,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    ) -> Object {
        let condition = self.evaluate_expression(condition);
        if let Object::Error(_) = condition {
            return condition;
        }
        if Self::is_truthy(condition) {
            return self.evaluate_block_statement(consequence);
        }

        if let Some(alternative) = alternative {
            return self.evaluate_block_statement(alternative);
        }

        Object::Null
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

    fn evaluate_identifier(&mut self, name: String) -> Object {
        match self.env.get(&name) {
            Some(obj) => obj,
            None => Object::Error(format!("identifier not found: {}", name)),
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
    use crate::{
        environment::Environment, evaluator::Evaluator, lexer::Lexer, object::Object,
        parser::Parser,
    };

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
            ("foobar", "identifier not found: foobar"),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object.to_string(), format!("Error: {}", test.1))
        }
    }

    #[test]
    fn test_let_statement() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Int(test.1));
        }
    }

    #[test]
    fn test_evaluate_function() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];

        for test in tests {
            let object = test_evaluate(test.0);
            assert_eq!(object, Object::Int(test.1));
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2 };";
        let evaluated = test_evaluate(input);
        if let Object::Function {
            parameters,
            body,
            environment: _,
        } = evaluated
        {
            assert_eq!(parameters[0], "x");
            assert_eq!(body.to_string(), "(x + 2)")
        }
    }

    fn test_evaluate(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut env = Environment::new();
        let mut evaluator = Evaluator::new(&mut env);
        evaluator.evaluate(program)
    }
}
