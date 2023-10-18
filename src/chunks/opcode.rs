use std::fmt::Display;

use num_derive::FromPrimitive;

use crate::chunks::Byte;

#[derive(Debug, Clone, FromPrimitive)]
pub enum Opcode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl Opcode {
    pub fn show(&self) -> String {
        ("OP_".to_owned() + format!("{:?}", self).as_str()).to_uppercase()
    }
}

impl From<Byte> for Opcode {
    fn from(byte: Byte) -> Self {
        let maybe_code = num::FromPrimitive::from_u8(byte);
        match maybe_code {
            Some(code) => code,
            _ => panic!("Invalid opcode value: {byte}")
        }
    }
}

impl From<Opcode> for Byte {
    fn from(opcode: Opcode) -> Self {
        opcode as Byte
    }
}