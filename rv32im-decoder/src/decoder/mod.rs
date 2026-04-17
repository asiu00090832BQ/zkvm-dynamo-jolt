pub mod base;
pub mod m_extension;

use crate::{
    encoding,
    error::ZkvmError,
    instruction::{Instruction, Register},
};

pub struct Decoder;

impl Decoder {
    pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
        decode(word)
    }
}

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    if (word & 0b11) != 0b11 {
        return Err(ZkvmError::InvalidInstruction(word));
    }

    if encoding::opcode(word) == encoding::OPCODE_OP && encoding::funct7(word) == encoding::FUNCT7_M {
        m_extension::decode(word)
    } else {
        base::decode(word)
    }
}

pub(crate) fn rd(word: u32) -> Result<Register, ZkvmError> {
    Register::try_from(encoding::rd(word))
}

pub(crate) fn rs1(word: u32) -> Result<Register, ZkvmError> {
    Register::try_from(encoding::rs1(word))
}

pub(crate) fn rs2(word: u32) -> Result<Register, ZkvmError> {
    Register::try_from(encoding::rs2(word))
}
