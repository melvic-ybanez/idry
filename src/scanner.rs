use std::str;

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
                '"' => self.scan_string(),
                c if is_digit(c) => self.scan_number(),
                c if is_alpha(c) => self.scan_identifier(),
                _ => Token::error("Unexpected character.", self)
            }
        }
    }

    fn scan_string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.next_line(); }
            self.advance();
        }

        if self.is_at_end() { Token::error("Unterminated string", self) } else {
            self.advance();     // consume the closing quote
            self.make_token(TokenType::Literal(Literal::Str))
        }
    }

    fn scan_number(&mut self) -> Token {
        // scan the whole number part
        while is_digit(self.peek()) {
            self.advance();
        }

        // scan the fractional part, if any
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance(); // consume the '.'
            while is_digit(self.peek()) { self.advance(); }
        }

        self.make_token(TokenType::Literal(Literal::Number))
    }

    fn scan_identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        self.make_token(self.identifier_type())
    }

    fn identifier_type(&self) -> TokenType {
        match self.source_at(self.start) {
            'a' => self.check_keyword(1, "nd", Keyword::And),
            'o' => self.check_keyword(1, "or", Keyword::Or),
            'c' => self.check_keyword(1, "lass", Keyword::Class),
            'e' => self.check_keyword(1, "lse", Keyword::Else),
            't' => self.check_keyword(1, "rue", Keyword::True),
            'w' => self.check_keyword(1, "hile", Keyword::While),
            'd' => self.check_keyword(1, "ef", Keyword::Def),
            'r' => self.check_keyword(1, "eturn", Keyword::Return),
            's' => self.check_keyword(1, "elf", Keyword::Self_),

            // for any branching path, check for the existence of a second letter
            // a single letter lexeme is still a valid identifier
            'f' if self.current_length() > 1 =>
                match self.source_at(self.start + 1) {
                    'a' => self.check_keyword(2, "lse", Keyword::False),
                    'o' => self.check_keyword(2, "r", Keyword::For),
                    _ => TokenType::Identifier
                }
            'i' if self.current_length() > 1 && self.source_at(self.start + 1) == 'm' =>
                self.check_keyword(2, "port", Keyword::Import),
            'i' => self.check_keyword(1, "f", Keyword::If),
            'l' if self.current_length() > 1 =>
                match self.source_at(self.start + 1) {
                    'e' => self.check_keyword(2, "t", Keyword::Let),
                    'a' => self.check_keyword(2, "mbda", Keyword::Lambda),
                    _ => TokenType::Identifier
                }
            'n' if self.current_length() > 1 && self.source_at(self.start + 1) == 'o' =>
                match self.source_at(self.start + 2) {
                    'n' => self.check_keyword(3, "e", Keyword::None),
                    _ => self.check_keyword(2, "t", Keyword::Let)
                }
            _ => TokenType::Identifier
        }
    }

    fn check_keyword(&self, start: usize, rest: &str, keyword_type: Keyword) -> TokenType {
        // We need to check if the lengths are the same, because the lexeme will be derived from
        // the keyword's length, which means we might only get a subset of the lexeme if the
        // keyword is shorter than the lexeme. Without this additional check, the comparison might
        // return true if the lexeme contains but is not equal to the keyword.
        let same_length = self.current_length() == start + rest.len();

        let same_rest = {
            let lexeme_rest = str::from_utf8(&self.source.as_bytes()[self.start + start..rest.len()])
                .unwrap();
            lexeme_rest == rest
        };

        if same_length && same_rest {
            TokenType::Keyword(keyword_type)
        } else {
            TokenType::Identifier
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

    fn current_length(&self) -> usize {
        self.current - self.start
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self)
    }

    fn make_token_or_else(&mut self, expected: char, if_match: TokenType, or_else: TokenType) -> Token {
        let token_type = if self.match_char(expected) { if_match } else { or_else };
        self.make_token(token_type)
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}