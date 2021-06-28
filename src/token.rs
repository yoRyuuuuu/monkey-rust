pub fn look_up_Ident(ident: &str) -> TokenKind {
    match ident {
        "let" => TokenKind::Let,
        "fn" => TokenKind::Function,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "return" => TokenKind::Return,
        "false" => TokenKind::False,
        "true" => TokenKind::True,
        _ => TokenKind::Ident
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Slash,
    Aster,
    Bang,

    Semicolon,
    Rparen,
    Lparen,
    Rbrace,
    Lbrace,
    Comma,

    Let,
    Function,
    True,
    False,
    If,
    Else,
    Return,

    GreaterThan,
    LessThan,

    Equal,
    NotEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}



