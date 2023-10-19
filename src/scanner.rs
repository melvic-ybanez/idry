use crate::common::Line;
use crate::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: Line,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner { source: source.to_string(), start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() {
            Token::new(TokenType::Eof, self)
        } else {
            Token::error("Unexpected character.", self)
        }
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
}