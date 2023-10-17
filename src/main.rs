use std::io;
use std::io::{BufRead, Read, Write as OtherWrite};

use crate::chunks::Write;

mod chunks;
mod vm;
mod disassemble;

fn main() {
    let args = std::env::args();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_script();
    } else {
        eprint!("Usage: dry [path]\n");
    }
}

fn repl() {
    loop {
        print!("dry> ");
        io::stdout().flush().expect("Can't flush stdout");

        let line = io::stdin().lock().lines().next()
            .expect("There's no next line")
            .expect("Unable to read next line");

        if line == "exit" {
            break;
        }

        interpret(line.as_str());
    }
}

fn run_script() {
    // TODO: Implement running of script
}

fn interpret(line: &str) {
    // TODO: Interpret lines
}
