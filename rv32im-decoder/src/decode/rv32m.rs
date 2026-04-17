use crate::{decoded::DecodedInstruction, error::DecodeError, format::InstructionFormat, instruction::Instruction, selectors::*};
pub fn decode(word: u32) -> Result<DecodedInstruction, DecodeError> {
    let inst = match funct3(word) {
        0 => Instruction::Mul, 1 => Instruction::Mulh, 2 => Instruction::Mulhsu, 3 => Instruction::Mulhu,
        4 => Instruction::Div, 5 => Instruction::Divu, 6 => Instruction::Rem, 7 => Instruction::Remu,
        _ => return Err(DecodeError::UnsupportedOpcode(0x33)),
    };
    Ok(DecodedInstruction::new(word, inst, InstructionFormat::R).with_rd(rd(word)).with_rs1(rs1(word)).with_rs2(rs2(word)))
}
