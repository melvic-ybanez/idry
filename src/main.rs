use chunks::Chunk;

use crate::chunks::opcode::Opcode;
use crate::chunks::Write;
use crate::vm::VM;

mod chunks;
mod vm;
mod disassemble;

fn main() {
    let mut chunk = Chunk::default();

    let constant_index = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant, 123);
    chunk.write(constant_index, 123);

    let constant_index = chunk.add_constant(3.4);
    chunk.write(Opcode::Constant, 123);
    chunk.write(constant_index, 123);

    chunk.write(Opcode::Add, 123);

    let constant_index = chunk.add_constant(5.6);
    chunk.write(Opcode::Constant, 123);
    chunk.write(constant_index, 123);

    chunk.write(Opcode::Divide, 123);
    chunk.write(Opcode::Negate, 123);

    chunk.write(Opcode::Return, 123);
    disassemble::chunk(&chunk, "test chunks");

    VM::interpret(chunk);
}
