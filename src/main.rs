use std::{fs, io, process};
use std::io::{BufRead, Read, Write as OtherWrite};
use std::path::Path;

use crate::chunks::Write;
use crate::vm::VmResult;

mod chunks;
mod vm;
mod disassemble;
mod compiler;
mod scanner;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_script(args[1].as_str());
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

        vm::interpret(line.as_str());
    }
}

fn run_script(path_string: &str) {
    let source = fs::read_to_string(Path::new(path_string)).expect(("Unable to read the file: ".to_owned() + path_string).as_str());
    let result = vm::interpret(source.as_str());

    match result {
        VmResult::CompileError => process::exit(65), // input data was incorrect
        VmResult::RuntimeError => process::exit(70), // internal software error
        VmResult::Ok => ()
    }
}
