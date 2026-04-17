use crate::error::{Result, ZkvmError};
use crate::m_extension as mext;

pub const MEXT_FUNCT7: u8 = 0x01;
pub const OP: u8 = 0x33;

pub fn decode(word: u32) -> Result<crate::types::Instruction> {
    let opcode = (word & 0x7f) as u8;
    let funct7 = ((word >> 25) & 0x7f;

    if opcode == OP && funct7 == MEXT_FUNCT7 {
        mext::decode(word).map(crate::types::Instruction::M)
    } else {
        Err(ZKvmError::UnknownInstruction { word })
    }
}
