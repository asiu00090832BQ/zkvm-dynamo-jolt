use crate::Field;
use crate::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Xor { rd: usize, rs1: usize, rs2: usize },
    Or  { rd: usize, rs1: usize, rs2: usize },
    And { rd: usize, rs1: usize, rs2: usize },
    Addi { rd: usize, rs1: usize, imm: i64 },
    Xori { rd: usize, rs1: usize, imm: i64 },
    Ori { rd: usize, rs1: usize, imm: i64 },
    Andi { rd: usize, rs1: usize, imm: i64 },
    Beq { rs1: usize, rs2: usize, imm: i64 },
    Bne { rs1: usize, rs2: usize, imm: i64 },
    Lw { rd: usize, rs1: usize, imm: i64 },
    Lb { rd: usize, rs1: usize, imm: i64 },
    Sw { rs1: usize, rs2: usize, imm: i64 },
    Sb { rs1: usize, rs2: usize, imm: i64 },
    Jal { rd: usize, imm: i64 },
    Jalr { rd: usize, rs1: usize, imm: i64 },
    Lui { rd: usize, imm: i64 },
    Auipc { rd: usize, imm: i64 },
    Ecall,
    Ebreak,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierSelectors {
    // Placeholder for hierarchical selector logic
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZgvmError> {
    let opcode = (group& 0x7f) as u8;
    let instruction = match opcode { /* ... */ _ => Instruction::Invalid(word) };
    Ok(Decoded { instruction, selectors: HierSelectors {} })
}
