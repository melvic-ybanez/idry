use crate::chunks::Chunk;
use crate::chunks::opcode::Opcode;

pub fn chunk(chunk: &Chunk, name: &str) {
    println!("== {name} ==");

    let mut offset = 0;

    while offset < chunk.code().len() {
        offset = instruction(chunk, offset);
    }
}

pub fn instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let lines = chunk.lines();

    if offset > 0 && lines[offset] == lines[offset - 1] {
        print!("   | ")
    } else {
        print!("{:4} ", lines[offset])
    }

    let instruction: &Opcode = &Opcode::from(chunk.code()[offset]);
    let name = instruction.show();
    match instruction {
        Opcode::Constant => chunk.constant_instruction(name.as_str(), offset),
        Opcode::Negate => simple_instruction(&name, offset),
        Opcode::Return => simple_instruction(&name, offset),
    }
}

fn simple_instruction(name: &str, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}