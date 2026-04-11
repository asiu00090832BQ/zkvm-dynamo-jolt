use crate::vm::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidInstruction(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub sub_op: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction { Add, Sub, Ecall, Ebreak, Invalid }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    Ok(Decoded {
        word,
        instruction: Instruction::Invalid,
        rd: 0, rs1: 0, rs2: 0, funct3: 0, funct7: 0,
        selectors: HierSelectors::default(),
    })
}