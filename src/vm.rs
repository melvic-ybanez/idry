use crate::chunks::{Byte, Chunk, values};
use crate::chunks::opcode::Opcode;
use crate::chunks::values::Value;
use crate::disassemble;

pub struct VM {
    chunk: Chunk,
    // Instruction pointer (or program counter). It points
    // to the next instruction to execute
    ip: usize,
}

impl VM {
    pub fn new(chunk: Chunk, ip: usize) -> Self {
        VM { chunk, ip }
    }

    pub fn interpret(chunk: Chunk) -> Result {
        let mut vm = VM::new(chunk, 0);
        vm.run()
    }

    fn run(&mut self) -> Result {
        loop {
            #[cfg(debug_assertions)]
            disassemble::instruction(&self.chunk, self.ip);

            let instruction = self.read_byte();
            match Opcode::from(instruction) {
                Opcode::Return => break Result::Ok,
                Opcode::Constant => {
                    let constant = self.read_constant();
                    values::print(constant);
                    println!();
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
}

pub enum Result {
    Ok,
    InterpreterError,
    RuntimeError,
}