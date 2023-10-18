use crate::scanner::{Line, Scanner, TokenType};

pub fn compile(source: &str) {
    let scanner = Scanner::new(source);
    let mut line: i32 = -1;
    loop {
        let token = scanner.scan_token();

        if token.line() != line as Line {
            print!("{:4} ", token.line());
            line = token.line() as i32;
        } else {
            print!("    | ");
        }

        print!("{:?} '{}'", token.kind(), token.lexeme());

        if *token.kind() == TokenType::Eof { break; }
    }
}