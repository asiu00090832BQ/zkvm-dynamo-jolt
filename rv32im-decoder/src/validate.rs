use crate::{
    error::DecodeError,
    instruction::{Instruction, OpImmKind},
};

pub fn validate(instruction: &Instruction) -> Result<(), DecodeError> {
    match instruction {
        Instruction::Lui { imm, .. } | Instruction::Auipc { imm, .. } => {
            if (*imm & 0x0fff) != 0 {
                return Err(DecodeError::Validation("U-immediate must have low 12 bits cleared"));
            }
        }
        Instruction::Jal { imm, .. } | Instruction::Branch { imm, .. } => {
            if (*imm & 0x1) != 0 {
                return Err(DecodeError::Validation("control-flow immediate must be 2-byte aligned"));
            }
        }
        Instruction::OpImm { kind, imm, .. } => {
            if matches!(*kind, OpImmKind::Slli | OpImmKind::Srli | OpImmKind::Srai)
                && !(0..=31).contains(imm)
            {
                return Err(DecodeError::Validation("shift amount must be in 0..=31"));
            }
        }
        _ => {}
    }
    Ok(())
}
