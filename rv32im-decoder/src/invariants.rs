use crate::error::DecodeError;
use crate::types::Instruction;

pub fn ensure_one_hot_slice(bits: &[u8], label: &'static str) -> Result<(), DecodeError> {
    let sum: u32 = bits.iter().map(|bit| *bit as u32).sum();
    if sum == 1 {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation(label))
    }
}

pub fn ensure_shift_range(shamt: u8) -> Result<(), DecodeError> {
    if shamt < 32 {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation("shift amount must be < 32"))
    }
}

pub fn ensure_even_offset(offset: i32, label: &'static str) -> Result<(), DecodeError> {
    if (offset & 1) == 0 {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation(label))
    }
}

pub fn ensure_limb16(
    value: u32,
    lo: u16,
    hi: u16,
    label: &'static str,
) -> Result<(), DecodeError> {
    let recomposed = ((hi as u32) << 16) | lo as u32;
    if value == recomposed {
        Ok(())
    } else {
        Err(DecodeError::InvariantViolation(label))
    }
}

pub fn validate_instruction(instruction: &Instruction) -> Result<(), DecodeError> {
    match instruction {
        Instruction::Slli { shamt, .. }
        | Instruction::Srli { shamt, .. }
        | Instruction::Srai { shamt, .. } => ensure_shift_range(*shamt),

        Instruction::Beq { imm, .. }
        | Instruction::Bne { imm, .. }
        | Instruction::Blt { imm, .. }
        | Instruction::Bge { imm, .. }
        | Instruction::Bltu { imm, .. }
        | Instruction::Bgeu { imm, .. } => ensure_even_offset(*imm, "branch offset must be even"),

        Instruction::Jal { imm, .. } => ensure_even_offset(*imm, "jump offset must be even"),

        _ => Ok(()),
    }
}
