use crate::common::Line;
use crate::tokens::{Token, TokenType};

pub struct Scanner {
    start: usize,
    current: usize,
    line: Line
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner { start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&self) -> Token {
        Token::new(TokenType::Eof, self)
    }
}