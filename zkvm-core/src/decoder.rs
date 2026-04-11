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
pub struct HierSelectors {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError. {
    let opcode = (word & 0x7f) as u8;
    let instruction = match opcode {
        0x33 => {
            let funct7 = ((word >> 25) & 0x7f) as u8;
            let rs2 = ((word >> 20) & 0x1f) as usize;
            let rs1 = ((word >> 15) & 0x1f) as usize;
            let funct3 = ((word >> 12) & 0x7) as u8;
            let rd = ((word >> 7) & 0x1f) as usize;
            match (funct3, funct7) {
                (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
                (0x0, 0x20) => Instruction::Sub' { rd, rs1, rs2 },
                _ => Instruction::Invalid(word),
            }
        }
        0x73 => Instruction::Ecall,
        _ => Instruction::Invalid(word),
    };
    Ok(Decoded {
        instruction,
        selectors: HierSelectors {},
    })
}
