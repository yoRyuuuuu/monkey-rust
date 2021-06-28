use crate::ast::{
    Expression, Expression::*, Precedence, Precedence::*, Program, Statement, Statement::*,
};

use crate::errors::ParserError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer: lexer,
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

    fn parse_program(&mut self) -> Result<Program> {
        let mut statements: Vec<Statement> = vec![];

        while !self.cur_token_is(TokenKind::Eof) {
            let mut stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }

        Ok(Program {
            statements: statements,
        })
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
            return Err(ParserError::TokenInvalid(TokenKind::Ident, self.cur_token.clone()).into());
        }
        let ident = Expression::Ident(self.cur_token.literal.clone());
        if !self.expect_peek(TokenKind::Assign) {
            return Err(
                ParserError::TokenInvalid(TokenKind::Assign, self.peek_token.clone()).into(),
            );
        }
        while !self.cur_token_is(TokenKind::Semicolon) {
            self.next_token();
        }
        let stmt = Statement::LetStatement {
            ident: ident,
            value: Expression::Ident("".to_string()),
        };
        Ok(stmt)
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        self.next_token();
        while !self.cur_token_is(TokenKind::Semicolon) {
            self.next_token();
        }
        let stmt = ReturnStatement(Ident("".to_string()));
        Ok(stmt)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement> {
        let exp = self.parse_expression(LOWEST)?;
        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }
        let stmt = ExpressionStatement(exp);
        Ok(stmt)
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression> {
        let op = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        let expression = Expression::Prefix {
            op: op,
            right: Box::new(right),
        };

        Ok(expression)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression> {
        let left_exp = match &self.cur_token.kind {
            TokenKind::Ident => self.parse_prefix(),
            TokenKind::Int => self.parse_int(),
            _ => Err(anyhow!("parse_expression()")),
        };

        left_exp
    }

    fn parse_prefix(&mut self) -> Result<Expression> {
        match self.cur_token.kind {
            TokenKind::Ident => self.parse_identifier(),
            TokenKind::Int => self.parse_int(),
            _ => panic!("parse_prefix()"),
        }
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

    fn cur_token_is(&self, tok: TokenKind) -> bool {
        self.cur_token.kind == tok
    }
    fn peek_token_is(&self, tok: TokenKind) -> bool {
        self.peek_token.kind == tok
    }
    fn expect_peek(&mut self, tok: TokenKind) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

mod tests {
    use crate::ast::{Expression::*, Statement, Statement::*};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use anyhow::Result;

    #[test]
    fn test_let_statements() {
        let input = r"let x = 5;
        let y = 10;
        let foobar = 838383;";

        let ident_tests = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);

        for (i, t) in ident_tests.into_iter().enumerate() {
            test_let_statement(&t, &program.statements[i]);
        }
    }

    fn test_let_statement(t: &String, s: &Statement) {
        match s {
            LetStatement {
                ref ident,
                value: _,
            } => match ident {
                Ident(ref str) => {
                    assert_eq!(str, t);
                }
                _ => panic!("Expression is not Ident"),
            },
            _ => panic!("Statement is not LetStatement"),
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r"return 5;
        return 10;
        return 993322;";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);

        for i in 0..3 {
            match program.statements[i] {
                ReturnStatement(_) => {}
                _ => panic!("Statement is not ReturnStatement"),
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = r#"foobar;"#;
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = program.statements[0].clone();
        match stmt {
            ExpressionStatement(exp) => match exp {
                Ident(str) => assert_eq!(str, "foobar"),
                _ => (),
            },
            _ => (),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = r#"5;"#;
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0].clone();
        match stmt {
            ExpressionStatement(exp) => match exp {
                Int(num) => assert_eq!(num, 5),
                _ => (),
            },
            _ => (),
        }
    }

    fn test_parsing_prefix_expression() -> Result<()> {
        let prefix_tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for test in prefix_tests {
            let lexer = Lexer::new(test.0);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program()?;
            assert_eq!(program.statements.len(), 1);
        }

        Ok(())
    }
}
