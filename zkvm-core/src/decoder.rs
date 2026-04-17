use crate::vm::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: u32, rs1: u32, rs2: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Ecall,
    Invalid(u32),
}

pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    Ok(Decoded { word, instruction: Instruction::Ecall })
}
