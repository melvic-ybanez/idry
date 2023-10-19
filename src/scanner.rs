use crate::common::Line;
use crate::tokens::{Token, TokenType};
use crate::tokens::Arithmetic;

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: Line,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner { source: source.to_string(), start: 0, current: 0, line: 1 }
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            self.make_token(TokenType::Eof)
        } else {
            let c = self.advance();
            match c {
                '(' => self.make_token(TokenType::LeftParen),
                ')' => self.make_token(TokenType::RightParen),
                '{' => self.make_token(TokenType::LeftBrace),
                '}' => self.make_token(TokenType::RightBrace),
                ',' => self.make_token(TokenType::Comma),
                '.' => self.make_token(TokenType::Dot),
                '+' => self.make_token(TokenType::Arithmetic(Arithmetic::Plus)),
                '*' => self.make_token(TokenType::Arithmetic(Arithmetic::Times)),
                '%' => self.make_token(TokenType::Arithmetic(Arithmetic::Modulo)),
                '-' => self.make_token(TokenType::Arithmetic(Arithmetic::Minus)),
                ';' => self.make_token(TokenType::Semicolon),
                _ => Token::error("Unexpected character.", self)
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn line(&self) -> Line {
        self.line
    }

    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self)
    }
}