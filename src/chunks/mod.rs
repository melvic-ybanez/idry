use crate::chunks::opcode::Opcode;
use crate::chunks::values::{Value, Values};

pub mod values;
pub mod opcode;

pub type Byte = u8;
pub type Code = Vec<Byte>;

pub trait Write<A> {
    fn write(&mut self, value: A, line: u32);
}

#[derive(Debug)]
pub struct Chunk {
    code: Code,
    constants: Values,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new(code: Code) -> Self {
        Self { code, constants: Values::default(), lines: vec![] }
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {name} ==");

        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value(value);
        self.constants.count() - 1
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);

        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ")
        } else {
            print!("{:4} ", self.lines[offset])
        }

        let instruction: &Opcode = &self.code[offset].into();
        let name = instruction.show();
        match instruction {
            Opcode::Constant => self.constant_instruction(name.as_str(), offset),
            Opcode::Return => {
                print!("{}", name);
                offset + 1
            }
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        // constant index is seated next to the constant opcode
        let constant_index = self.code[offset + 1];
        print!("{:16} {:4} '", name, constant_index);
        values::print(self.constants.values[constant_index as usize]);
        println!("'");

        // we are moving two offsets forward (1 for the opcode
        // and another for the constant)
        offset + 2
    }

    pub fn code(&self) -> &Code {
        &self.code
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Chunk::new(vec![])
    }
}

impl Write<Opcode> for Chunk {
    fn write(&mut self, opcode: Opcode, line: u32) {
        let opcode_byte: Byte = opcode.into();
        self.write(opcode_byte, line);
    }
}

impl Write<Byte> for Chunk {
    fn write(&mut self, byte: Byte, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }
}

impl Write<usize> for Chunk {
    fn write(&mut self, value: usize, line: u32) {
        self.write(value as Byte, line);
    }
}
