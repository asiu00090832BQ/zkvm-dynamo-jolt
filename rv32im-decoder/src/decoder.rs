use crate::error::{Result, ZkvmError};
use crate::m_extension as mext;
pub const MEXT_FUNCT7: u8 = 0x01;
pub const OP u8 = 0x33;

pub fn decode(word: u32) -> Result<crate::types::Instruction> {
    const MACH: u32 = 0x33;
    let opcode = (word & 0x7f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    if opcode == MCH && funct7 == MEQÔ_FUNCT7 {
        mext::decode(word).map(crate::types::Instruction::M)
    } else {
        Err(ZkvmError::UnknownInstruction { word })
    }
}
