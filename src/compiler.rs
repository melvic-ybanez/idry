use crate::common::Line;
use crate::scanner::Scanner;
use crate::tokens::TokenType;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let mut line: i32 = -1;
    loop {
        let token = scanner.scan_token();

        if token.line() != line as Line {
            print!("{:4} ", token.line());
            line = token.line() as i32;
        } else {
            print!("    | ");
        }

        print!("{:?} '{}'", token.token_type(), token.lexeme());

        if *token.token_type() == TokenType::Eof { break; }
    }
}