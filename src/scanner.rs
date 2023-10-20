use std::thread::current;
use crate::common::Line;
use crate::tokens::{Bitwise, Comparison, Keyword, Literal, Token, TokenType};
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
        self.skip_whitespace();
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
                '!' => self.make_token_or_else(
                    '=', TokenType::Comparison(Comparison::NotEqual),
                    TokenType::Keyword(Keyword::Not)),
                '=' => self.make_token_or_else(
                    '=', TokenType::Comparison(Comparison::EqualEqual),
                    TokenType::Comparison(Comparison::Equal)),
                '&' => self.make_token(TokenType::Bitwise(Bitwise::BAnd)),
                '|' => self.make_token(TokenType::Bitwise(Bitwise::BOr)),
                '^' => self.make_token(TokenType::Bitwise(Bitwise::BXor)),
                '"' => self.string(),
                _ => Token::error("Unexpected character.", self)
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.next_line(); }
            self.advance();
        }

        if self.is_at_end() { Token::error("Unterminated string", self) }
        else {
            self.advance();     // consume the closing quote
            self.make_token(TokenType::Literal(Literal::Str))
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    fn peek(&self) -> char {
        self.source_at(self.current)
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() { '\0' } else { self.source_at(self.current + 1) }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.peek() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => { self.advance(); }
                '\n' => {
                    self.next_line();
                    self.advance();
                }
                '/' => if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else { break; }
                _ => break
            }
        }
    }

    fn next_line(&mut self) {
        self.line += 1;
    }

    fn source_at(&self, index: usize) -> char {
        self.source.as_bytes()[index] as char
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

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self)
    }

    fn make_token_or_else(&mut self, expected: char, if_match: TokenType, or_else: TokenType) -> Token {
        let token_type = if self.match_char(expected) { if_match } else { or_else };
        self.make_token(token_type)
    }
}