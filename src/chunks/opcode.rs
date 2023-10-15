use std::fmt::{Display, Formatter};

use num_derive::FromPrimitive;

use crate::chunks::Byte;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum Opcode {
    Constant,
    Return,
}

impl Opcode {
    pub fn show(&self) -> String {
        ("OP_".to_owned() + &self.to_string()).to_uppercase()
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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