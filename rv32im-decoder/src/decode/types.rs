use crate::{error::DecodeError, register::Register};

[derive(Debug, Copy, Clone, PartialEq, Eq)]]
pub(crate) struct DecodedFields {
    pub word: u32,
    pub opcode: u8,
    pub rd: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct7: u8,
}

impl From<u32> for DecodeFields {
    fn from(word: u32) -> Self {
        Self {
            word,
            opcode: (word & 0x7f) as u8,
            rd: ((word >> 7) & 0x1f) as u8,
            funct3: ((word >> 12) & 0x07) as u8,
            rs1: ((word >> 15) & 0x1f) as u8,
            rs2: ((word >> 20) & 0x1f) as u8,
            funct7: ((word >> 25) & 0x7f) as u8,
        }
    }
}

impl DecodedFields {
    pub fn rd_reg(&self) -> Result<Register, DecodeError> {
        self.rd.try_into()
    }

    pub fn rs1_reg(&self) -> Result<Register, DecodeError> {
        self.rs1.try_into()
    }

    pub fn rs2_reg(&self) -> Result<Register, DecodeError> {
        self.rs2.try_into()
    }
}
