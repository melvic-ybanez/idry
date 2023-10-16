use crate::chunks::{Byte, Chunk, values};
use crate::chunks::opcode::Opcode;
use crate::chunks::values::Value;
use crate::disassemble;
use crate::vm::VmResult::InterpreterError;

pub struct VM {
    chunk: Chunk,
    // Instruction pointer (or program counter). It points
    // to the next instruction to execute
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Chunk) -> Self {
        VM { chunk, ip: 0, stack: vec![] }
    }

    pub fn interpret(chunk: Chunk) -> VmResult {
        let mut vm = VM::new(chunk);
        vm.run()
    }

    fn run(&mut self) -> VmResult {
        loop {
            #[cfg(debug_assertions)] {
                print!("          ");
                self.stack.iter().for_each(|slot| {
                    print!("[ {} ]", slot);
                });
                println!();
                disassemble::instruction(&self.chunk, self.ip);
            }

            let instruction = self.read_byte();
            match Opcode::from(instruction) {
                Opcode::Return => {
                    values::print(self.pop_unsafe());
                    println!();
                    break VmResult::Ok
                }
                Opcode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                    println!();
                }
                Opcode::Negate => {
                    let pop = -self.pop_unsafe();
                    self.push(pop);
                }
            }
        }
    }

    fn read_byte(&mut self) -> Byte {
        let instruction = self.chunk.code()[self.ip];
        self.ip += 1;
        instruction
    }

    fn read_constant(&mut self) -> Value {
        let offset = self.read_byte() as usize;
        self.chunk.constants().values()[offset]
    }

    fn reset_stack(&mut self) {
        // Note: if this proves to be inefficient, consider implementing a
        // stack using arrays and track the top using and index. That way
        // we can reuse the stack for new values by simply resetting the index.
        self.stack.clear();
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    fn pop_unsafe(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn pop_with<F>(&mut self, f: F) -> VmResult
        where F: Fn(Value) -> VmResult {
        self.pop().map(f).unwrap_or(InterpreterError)
    }
}

pub enum VmResult {
    Ok,
    InterpreterError,
    RuntimeError,
}