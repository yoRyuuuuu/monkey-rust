use crate::ast::{BlockStatement, Expression, Precedence, Program, Statement};

use crate::errors::MonkeyError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Token {
                kind: TokenKind::Eof,
                literal: String::from(""),
            },
            peek_token: Token {
                kind: TokenKind::Eof,
                literal: String::from(""),
            },
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program> {
        let mut statements: Vec<Statement> = vec![];

        while !self.cur_token_is(TokenKind::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement> {
        match self.cur_token.kind {
            TokenKind::Let => Ok(self.parse_let_statement()?),
            TokenKind::Return => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement> {
        if !self.expect_peek(TokenKind::Ident) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Ident, self.cur_token.clone()).into(),
            );
        }

        let ident = Expression::Ident(self.cur_token.literal.clone());
        if !self.expect_peek(TokenKind::Assign) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Assign, self.peek_token.clone()).into(),
            );
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let stmt = Statement::Let { ident, value };
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.next_token();
        let value = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let stmt = Statement::Return(value);
        Ok(stmt)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let expr = self.parse_expression(Precedence::Lowest)?;
        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        let stmt = Statement::Expression(expr);
        Ok(stmt)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression> {
        let op = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix)?;

        let expression = Expression::Prefix {
            op,
            right: Box::new(right),
        };
        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
        let mut left_expr = match &self.cur_token.kind {
            TokenKind::Ident => self.parse_prefix(),
            TokenKind::Int => self.parse_int(),
            TokenKind::True | TokenKind::False => self.parse_boolean(),
            TokenKind::Bang | TokenKind::Minus => self.parse_prefix_expression(),
            TokenKind::Lparen => self.parse_group_expression(),
            TokenKind::If => self.parse_if_expression(),
            TokenKind::Function => self.parse_function_literal(),
            _ => return Err(MonkeyError::InvalidToken(self.cur_token.clone()).into()),
        }?;

        while !self.peek_token_is(TokenKind::Semicolon) && precedence < self.peek_precedence() {
            left_expr = match self.peek_token.kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Aster
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::GreaterThan => {
                    self.next_token();
                    self.parse_infix_expression(left_expr)?
                }
                TokenKind::Lparen => {
                    self.next_token();
                    self.parse_call_expression(left_expr)?
                }
                _ => left_expr,
            };
        }

        Ok(left_expr)
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression> {
        let arguments = self.parse_call_arguments()?;
        let expr = Expression::Call {
            function: Box::new(function),
            arguments,
        };

        Ok(expr)
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut args = vec![];

        if self.peek_token_is(TokenKind::Rparen) {
            self.next_token();
            return Ok(args);
        }

        self.next_token();
        args.push(self.parse_expression(Precedence::Lowest)?);

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(Precedence::Lowest)?);
        }

        if !self.expect_peek(TokenKind::Rparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Rparen, self.cur_token.clone()).into(),
            );
        }

        Ok(args)
    }

    fn parse_function_literal(&mut self) -> Result<Expression> {
        if !self.expect_peek(TokenKind::Lparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Lparen, self.cur_token.clone()).into(),
            );
        }

        let parameters = self.parse_function_parameters()?;

        if !self.expect_peek(TokenKind::Lbrace) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Lbrace, self.cur_token.clone()).into(),
            );
        }

        let body = self.parse_block_statement()?;

        let func = Expression::Function { parameters, body };

        Ok(func)
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<String>> {
        let mut idents = vec![];

        if self.peek_token_is(TokenKind::Rparen) {
            self.next_token();
            return Ok(idents);
        }

        self.next_token();

        let ident = self.cur_token.literal.clone();
        idents.push(ident);

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();
            let ident = self.cur_token.literal.clone();
            idents.push(ident);
        }

        if !self.expect_peek(TokenKind::Rparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Rparen, self.cur_token.clone()).into(),
            );
        }

        Ok(idents)
    }

    fn parse_if_expression(&mut self) -> Result<Expression> {
        if !self.expect_peek(TokenKind::Lparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Lparen, self.peek_token.clone()).into(),
            );
        }

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(TokenKind::Rparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Rparen, self.peek_token.clone()).into(),
            );
        }

        if !self.expect_peek(TokenKind::Lbrace) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Lbrace, self.peek_token.clone()).into(),
            );
        }

        let consequence = self.parse_block_statement()?;
        let mut alternative = None;

        if self.peek_token_is(TokenKind::Else) {
            self.next_token();

            if !self.expect_peek(TokenKind::Lbrace) {
                return Err(MonkeyError::UnexpectedToken(
                    TokenKind::Lbrace,
                    self.peek_token.clone(),
                )
                .into());
            }

            alternative = self.parse_block_statement()?.into();
        }

        let expr = Expression::If {
            condition: Box::new(condition),
            consequence,
            alternative,
        };

        Ok(expr)
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement> {
        self.next_token();
        let mut statements = vec![];
        while !self.cur_token_is(TokenKind::Rbrace) && !self.cur_token_is(TokenKind::Eof) {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        let block = BlockStatement { statements };

        Ok(block)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression> {
        let op = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;
        let expr = Expression::Infix {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };

        Ok(expr)
    }

    fn parse_prefix(&mut self) -> Result<Expression> {
        match self.cur_token.kind {
            TokenKind::Ident => self.parse_identifier(),
            TokenKind::Int => self.parse_int(),
            _ => panic!("parse_prefix()"),
        }
    }

    fn parse_group_expression(&mut self) -> Result<Expression> {
        self.next_token();
        let expr = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(TokenKind::Rparen) {
            return Err(
                MonkeyError::UnexpectedToken(TokenKind::Rparen, self.cur_token.clone()).into(),
            );
        }

        expr
    }

    fn parse_identifier(&self) -> Result<Expression> {
        Ok(Expression::Ident(self.cur_token.literal.clone()))
    }

    fn parse_int(&self) -> Result<Expression> {
        match self.cur_token.literal.clone().parse::<i64>() {
            Ok(num) => Ok(Expression::Int(num)),
            Err(err) => Err(err.into()),
        }
    }

    fn parse_boolean(&self) -> Result<Expression> {
        Ok(Expression::Boolean(self.cur_token_is(TokenKind::True)))
    }

    fn cur_token_is(&self, tok: TokenKind) -> bool {
        self.cur_token.kind == tok
    }

    fn peek_token_is(&self, tok: TokenKind) -> bool {
        self.peek_token.kind == tok
    }

    fn expect_peek(&mut self, tok: TokenKind) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn cur_precedence(&self) -> Precedence {
        self.cur_token.get_precedence()
    }

    fn peek_precedence(&self) -> Precedence {
        self.peek_token.get_precedence()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Program, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let {
                ident: Expression::Ident("myVar".to_string()),
                value: Expression::Ident("anotherVar".to_string()),
            }],
        };

        let stmt = program.statements[0].to_string();
        assert_eq!(stmt, "let myVar = anotherVar;");
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x } else { y }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let len = program.statements.len();
        assert_eq!(
            len, 1,
            "program.statements does not contain 1 statements. got={}",
            len
        );

        let stmt = program.statements[0].clone();

        let expr = match stmt {
            Statement::Expression(expr) => expr,
            _ => panic!(
                "program.statements[0] is not Statement::Expression. got={}",
                stmt
            ),
        };

        let (_, consequence, _) = match expr {
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                assert_eq!(condition.to_string(), "(x < y)");
                assert_eq!(consequence.to_string(), "x");
                if let Some(ref alternative) = alternative {
                    assert_eq!(alternative.to_string(), "y");
                }
                (*condition, consequence, alternative)
            }
            _ => panic!("expr is not Expression::If. got={}", expr),
        };

        assert_eq!(
            consequence.statements.len(),
            1,
            "consequence is not 1 statements. got={}",
            consequence.statements.len()
        )
    }

    #[test]
    fn test_return_statement() {
        let input = r#"return 5;
return 10;
return 993322;"#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let actual = parser.parse_program().unwrap();
        assert_eq!(actual.statements.len(), 3);
        let want = vec!["5", "10", "993322"];
        for (stmt, w) in actual.statements.iter().zip(want) {
            match stmt {
                Statement::Return(value) => assert_eq!(value.to_string(), w),
                _ => panic!(),
            }
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];

        for (input, expect) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let actual = parser.parse_program().unwrap().statements[0].to_owned();
            assert_eq!(actual.to_string(), expect);
        }
    }

    #[test]
    fn test_function_expression() {
        let input = "fn (x, y) { x + y }";
        let test = "fn (x, y) { (x + y) }";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let stmts = program.statements[0].clone();
        assert_eq!(stmts.to_string(), test);
    }
}
