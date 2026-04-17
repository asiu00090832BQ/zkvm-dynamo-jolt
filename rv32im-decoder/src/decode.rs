use crate::base_i::decode_base_i;
use crate::error::ZkvmError;
use crate::formats::{funct7, opcode};
use crate::instruction::Instruction;
use crate::m_extension::decode_m_extension;

pub fn decode_instruction(word: u32) -> Result<Instruction, ZkvmError> {
    match opcode(word) {
        0x33 if funct7(word) == 0x01 => decode_m_extension(word),
        0x33 | 0x13 | 0x37 | 0x17 | 0x6f | 0x67 | 0x63 | 0x03 | 0x23 => decode_base_i(word),
        _ => Err(ZkvmError::InvalidInstruction(word)),
    }
}
