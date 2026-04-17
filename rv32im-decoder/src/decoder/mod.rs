pub mod base; pub mod m_extension;
use crate::{encoding::{self, funct7, opcode}, error::ZkvmResult, instruction::Instruction};
pub fn decode(word: u32) -> ZkvmResult<Instruction> {
    if encoding::opcode(word) == opcode::OP && encoding::funct7(word) == funct7::M_EXTENSION { return m_extension::decode_m_extension(word); }
    base::decode_base(word)
}
