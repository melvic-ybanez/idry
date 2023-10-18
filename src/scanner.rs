pub type Line = u32;

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

pub struct Token {
    line: Line,
    kind: TokenType,
    start: usize,
    length: usize,
}

impl Token {
    pub fn new(kind: TokenType, scanner: &Scanner) -> Self {
        Token { line: 1, kind, start: 0, length: 0}
    }

    pub fn line(&self) -> Line {
        self.line
    }

    pub fn kind(&self) -> &TokenType {
        &self.kind
    }

    pub fn lexeme(&self) -> String {
        "".to_owned()
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof
}