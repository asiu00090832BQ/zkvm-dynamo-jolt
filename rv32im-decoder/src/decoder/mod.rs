pub mod base_i;
pub mod invariants;
pub mod m_extension;

use crate::error::Result;
use crate::fields::RawInstruction;
use crate::instruction::Instruction;

pub fn decode(word: u32) -> Result<Instruction> {
    invariants::validate_word(word)?;
    let raw = RawInstruction::new(word);
    invariants::validate_raw_registers(raw)?;

    if raw.opcode() == 0b0110011 && raw.funct7() == 0b0000001 {
        return m_extension::decode_m_extension(raw);
    }

    base_i::decode_base_i(raw)
}

pub fn decode_bytes(bytes: [u8; 4]) -> Result<Instruction> {
    decode(u32::from_le_bytes(bytes))
}
