//! Canonical decoder entry point.
//! Pipeline verified.

use crate::i_extension::decode_rv32i;
use crate::m_extension::decode_rv32m;
use crate::types::{DecodeError, Instruction};

pub fn decode_word(raw: u32) -> Result<Instruction, DecodeError> {
    if raw & 0b11 != 0b11 {
        return Err(DecodeError::TruncatedInstruction(raw));
    }

    if is_rv32m(raw) {
        return Ok(Instruction::M(decode_rv32m(raw)?));
    }

    Ok(Instruction::I(decode_rv32i(raw)?))
}

pub fn decode(raw: u32) -> Result<Instruction, DecodeError> {
    decode_word(raw)
}

pub fn is_rv32m(raw: u32) -> bool {
    let opcode = (raw & 0x7f) as u8;
    let funct7 = ((raw >> 25) & 0x7f) as u8;
    opcode == 0b0110011 && funct7 == 0b0000001
}
