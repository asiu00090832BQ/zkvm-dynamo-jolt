use crate::types::{DecodeError, DecodedInstruction, OPCODE_OP, OPCODE_OP_IMM, FUNCT7_M};
use crate::i_extension::decode_rv32i;
use crate::m_extension::decode_rv32m;

pub fn decode_word(raw: u32) -> Result<DecodedInstruction, DecodeError> {
    let opcode = (raw & 0x7f) as u8;
    match opcode {
        OPCODE_OP_IMM => Ok(DecodedInstruction::I(decode_rv32i(raw)?)),
        OPCODE_OP => {
            let funct7 = (raw >> 25) as u8;
            if funct7 == FUNCT7_M {
                Ok(DecodedInstruction::M(decode_rv32m(raw)?))
            } else {
                Err(DecodeError::UnsupportedOpcode(raw))
            }
        }
        _ => Err(DecodeError::UnsupportedOpcode(raw)),
    }
}

pub fn decode(raw: u32) -> Result<DecodedInstruction, DecodeError> {
    decode_word(raw)
}

pub fn is_rv32m(raw: u32) -> bool {
    let opcode = (raw & 0x7f) as u8;
    let funct7 = (raw >> 25) as u8;
    opcode == OPCODE_OP && funct7 == FUNCT7_M
}
