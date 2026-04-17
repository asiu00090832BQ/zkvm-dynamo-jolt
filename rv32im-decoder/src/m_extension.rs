use crate::error::DecodeError;
use crate::instruction::{Instruction, RTypeFields};

/// Lemma 6.1.1 (Hierarchical Multiplication Reduction) for M-extension decoding.
pub fn decode_m_extension(word: u32, funct3: u8, fields: RTypeFields) -> Result<Instruction, DecodeError> {
    match funct3 {
        0b000 => Ok(Instruction::Mul(fields)),
        0b001 => Ok(Instruction::Mulh(fields)),
        0b010 => Ok(Instruction::Mulhsu(fields)),
        0b011 => Ok(Instruction::Mulhu(fields)),
        0b100 => Ok(Instruction::Div(fields)),
        0b101 => Ok(Instruction::Divu(fields)),
        0b110 => Ok(Instruction::Rem(fields)),
        0b111 => Ok(Instruction::Remu(fields)),
        _ => Err(DecodeError::UnsupportedFunct3 {
            word,
            opcode: 0x33,
            funct3,
        }),
    }
}
