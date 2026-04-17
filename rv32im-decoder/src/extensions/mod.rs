pub mod i_extension;
pub mod m_extension;

use crate::types::{DecodedInstruction, ZkvmError};

#[inline]
pub fn decode(raw: u32) -> Result<DecodedInstruction, ZkvmError> {
    let opcode = (raw & 0x7f) as u8;
    let funct7 = ((raw >> 25) & 0x7f) as u8;

    if opcode == 0b0110011 && funct7 == 0b0000001 {
        return m_extension::decode(raw);
    }

    i_extension::decode(raw)
}
