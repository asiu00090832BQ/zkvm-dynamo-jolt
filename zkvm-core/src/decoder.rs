use crate::vm::ZkvmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Addi { rd: usize, rs1: usize, imm: i32 },
    Lui { rd: usize, imm: i32 },
    Jal { rd: usize, imm: i32 },
    Ecall,
    Ebreak,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub sub_op: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError> {
    if (word & 0x3) != 0x3 {
        return Err(ZkvmError::DecodeError);
    }
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    let (instruction, selectors) = match opcode {
        0x33 => {
            let inst = match (funct3, funct7) {
                (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
                (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 },
                _ => Instruction::Invalid(word),
            };
            (
                inst,
                HierSelectors {
                    is_alu: true,
                    is_system: false,
                    sub_op: funct3,
                },
            )
        }
        0x13 => {
            let imm = (word as i32) >> 20;
            let inst = match funct3 {
                0x0 => Instruction::Addi { rd, rs1, imm },
                _ => Instruction::Invalid(word),
            };
            (
                inst,
                HierSelectors {
                    is_alu: true,
                    is_system: false,
                    sub_op: funct3,
                },
            )
        }
        0x37 => {
            let imm = (word & 0xfffff000) as i32;
            (
                Instruction::Lui { rd, imm },
                HierSelectors::default(),
            )
        }
        0x6f => {
            let imm20 = (word >> 31) & 1;
            let imm10_1 = (word >> 21) & 0x3ff;
            let imm11 = (word >> 20) & 1;
            let imm19_12 = (word >> 12) & 0xff;
            let imm = ((imm20 as i32) << 20) | ((imm19_12 as i32) << 12) | ((imm11 as i32) << 11) | ((imm10_1 as i32) << 1);
            let imm = if imm20 != 0 { imm | !0xfffff } else { imm };
            (
                Instruction::Jal { rd, imm },
                HierSelectors::default(),
            )
        }
        0x73 => {
            let inst = match word {
                0x0000_0073 => Instruction::Ecall,
                0x0010_0073 => Instruction::Ebreak,
                _ => Instruction::Invalid(word),
            };
            (
                inst,
                HierSelectors {
                    is_alu: false,
                    is_system: true,
                    sub_op: 0,
                },
            )
        }
        _ => (Instruction::Invalid(word), HierSelectors::default()),
    };

    Ok(Decoded {
        word,
        instruction,
        selectors,
    })
}