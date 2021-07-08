use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

macro_rules! token {
    ($kind:expr, $literal:expr) => {
        Token {
            kind: $kind,
            literal: String::from($literal),
        }
    };
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position < self.input.len() {
            self.ch = self.input.as_bytes()[self.read_position];
        } else {
            self.ch = 0;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token!(TokenKind::Equal, "==")
                } else {
                    token!(TokenKind::Assign, "=")
                }
            }
            b'+' => token!(TokenKind::Plus, "+"),
            b'-' => token!(TokenKind::Minus, "-"),
            b'/' => token!(TokenKind::Slash, "/"),
            b'*' => token!(TokenKind::Aster, "*"),
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    token!(TokenKind::NotEqual, "!=")
                } else {
                    token!(TokenKind::Bang, "!")
                }
            }
            b';' => token!(TokenKind::Semicolon, ";"),
            b'(' => token!(TokenKind::Lparen, "("),
            b')' => token!(TokenKind::Rparen, ")"),
            b',' => token!(TokenKind::Comma, ","),
            b'{' => token!(TokenKind::Lbrace, "{"),
            b'}' => token!(TokenKind::Rbrace, "}"),
            b'>' => token!(TokenKind::GreaterThan, ">"),
            b'<' => token!(TokenKind::LessThan, "<"),
            b'a'..=b'z' | b'A'..=b'Z' => {
                let literal = self.read_identifier();
                let kind = crate::token::look_up_ident(&literal);
                return Token { kind, literal };
            }
            b'0'..=b'9' => {
                return Token {
                    kind: TokenKind::Int,
                    literal: self.read_number(),
                }
            }
            _ => token!(TokenKind::Eof, ""),
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while let b'a'..=b'z' | b'A'..=b'Z' = self.ch {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while let b'0'..=b'9' = self.ch {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }

    fn skip_whitespace(&mut self) {
        while let b' ' | b'\t' | b'\n' | b'\r' = self.ch {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position < self.input.len() {
            self.input.as_bytes()[self.read_position]
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenKind::*};

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
    let ten = 10;
    let add = fn(x, y) {
        x + y;
        };

        let result = add(five, ten);

        !-/*5;
        5 < 10 > 5;

        if (5 > 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;"#;

        let tests: Vec<Token> = vec![
            token!(Let, "let"),
            token!(Ident, "five"),
            token!(Assign, "="),
            token!(Int, "5"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "ten"),
            token!(Assign, "="),
            token!(Int, "10"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "add"),
            token!(Assign, "="),
            token!(Function, "fn"),
            token!(Lparen, "("),
            token!(Ident, "x"),
            token!(Comma, ","),
            token!(Ident, "y"),
            token!(Rparen, ")"),
            token!(Lbrace, "{"),
            token!(Ident, "x"),
            token!(Plus, "+"),
            token!(Ident, "y"),
            token!(Semicolon, ";"),
            token!(Rbrace, "}"),
            token!(Semicolon, ";"),
            token!(Let, "let"),
            token!(Ident, "result"),
            token!(Assign, "="),
            token!(Ident, "add"),
            token!(Lparen, "("),
            token!(Ident, "five"),
            token!(Comma, ","),
            token!(Ident, "ten"),
            token!(Rparen, ")"),
            token!(Semicolon, ";"),
            token!(Bang, "!"),
            token!(Minus, "-"),
            token!(Slash, "/"),
            token!(Aster, "*"),
            token!(Int, "5"),
            token!(Semicolon, ";"),
            token!(Int, "5"),
            token!(LessThan, "<"),
            token!(Int, "10"),
            token!(GreaterThan, ">"),
            token!(Int, "5"),
            token!(Semicolon, ";"),
            token!(If, "if"),
            token!(Lparen, "("),
            token!(Int, "5"),
            token!(GreaterThan, ">"),
            token!(Int, "10"),
            token!(Rparen, ")"),
            token!(Lbrace, "{"),
            token!(Return, "return"),
            token!(True, "true"),
            token!(Semicolon, ";"),
            token!(Rbrace, "}"),
            token!(Else, "else"),
            token!(Lbrace, "{"),
            token!(Return, "return"),
            token!(False, "false"),
            token!(Semicolon, ";"),
            token!(Rbrace, "}"),
            token!(Int, "10"),
            token!(Equal, "=="),
            token!(Int, "10"),
            token!(Semicolon, ";"),
            token!(Int, "10"),
            token!(NotEqual, "!="),
            token!(Int, "9"),
            token!(Semicolon, ";"),
        ];

        let mut lexer = Lexer::new(input);
        for t in tests {
            assert_eq!(lexer.next_token(), t);
        }
    }
}
