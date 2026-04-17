use crate::{bits::i_imm, decoded::DecodedInstruction, error::DecodeError, format::InstructionFormat, instruction::Instruction, selectors::*};
pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    match funct3(word) {
        0 => match csr(word) { 0 => Ok(DecodedInstruction::new(word, Instruction::Ecall, InstructionFormat::I)), 1 => Ok(DecodedInstruction::new(word, Instruction::Ebreak, InstructionFormat::I)), _ => Err(DecodeError::UnsupportedOpcode(0x73)) },
        _ => Err(DecodeError::UnsupportedOpcode(0x73)),
    }
}
