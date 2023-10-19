use crate::common::Line;
use crate::scanner::Scanner;

pub struct Token<'a> {
    line: Line,
    token_type: TokenType,
    lexeme: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, scanner: &'a Scanner) -> Self {
        Token {
            line: scanner.line(),
            token_type,
            lexeme: &scanner.source()[scanner.start()..scanner.current()],
        }
    }

    pub fn error(message: &'static str, scanner: &'a Scanner) -> Self {
        Token {
            token_type: TokenType::Error,
            lexeme: message,
            line: scanner.line(),
        }
    }

    pub fn line(&self) -> Line {
        self.line
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn lexeme(&self) -> String {
        "".to_owned()
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Arithmetic(Arithmetic),
    Bitwise(Bitwise),
    Comparison(Comparison),
    Literal(Literal),
    Keyword(Keyword),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Error,
    Eof,
}

#[derive(Debug, PartialEq)]
enum Arithmetic {
    Minus,
    Plus,
    Slash,
    Star,
    Modulo,
}

#[derive(Debug, PartialEq)]
enum Bitwise {
    BAnd,
    BOr,
    BXor,
    LeftShift,
    RightShift,
    URightShift,
}

#[derive(Debug, PartialEq)]
enum Comparison {
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, PartialEq)]
enum Literal {
    Identifier,
    Str,
    Number,
}

#[derive(Debug, PartialEq)]
enum Keyword {
    And,
    Not,
    Or,
    Class,
    Else,
    False,
    Def,
    For,
    If,
    None,
    Return,
    Self_,
    True,
    Let,
    While,
    Lambda,
    Import,
}