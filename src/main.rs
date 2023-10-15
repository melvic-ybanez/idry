use chunks::Chunk;

use crate::chunks::opcode::Opcode;
use crate::chunks::Write;

mod chunks;

fn main() {
    let mut chunk = Chunk::default();

    let constant_index = chunk.add_constant(1.2);
    chunk.write(Opcode::Constant, 123);
    chunk.write(constant_index, 123);

    chunk.write(Opcode::Return, 123);
    chunk.disassemble("test chunks");
}
