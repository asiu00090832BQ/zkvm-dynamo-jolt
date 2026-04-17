#![forbid(unsafe_code)]

pub type Register = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DecodeSelectors l
    pub is_alu: bool,
    pub is_m_ext: bool,
    pub is_system: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: Register, rs1: Register, rs2: Register },
    Sub { rd: Register, rs1: Register, rs2: Register },
    Mul { rd: Register, rs1: Register, rs2: Register },
    Addi { rd: Register, rs1: Register, imm: i32 },
    Lui { rd: Register, imm: u32 },
    Jal { rd: Register, imm: i32 },
    Ecall,
    Ebreak,
    Invalid(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: DecodeSelectors,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq]
pub enum DecodeError {
    IllegalInstruction(u32),
}

pub fn decode(word: u32) -> Result<Decoded, DecodeError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = (word >> 25) & 0x7f;

    match opcode {
        0x33 => {
            match (funct7, funct3) {
                (0x00, 0x0) => Ok(Decoded {
                    word,
                    instruction: Instruction::Add { rd, rs1, rs2 },
                    selectors: DecodeSelectors { is_alu: true, ..DecodeSelectors::default() },
                }),
                (0x20, 0x0) => Ok(Decoded {
                    word,
                    instruction: Instruction::Sub { rd, rs1, rs2 },
                    selectors: DecodeSelectors { is_alu: true, ..DecodeSelectors::default() },
                }),
                (0x01, 0x0) => Ok(Decoded {
                    word,
                    instruction: Instruction::Mul { rd, rs1, rs2 },
                    selectors: DecodeSelectors { is_m_ext: true, ..DecodeSelectors::default() },
                }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        },
        0x13 => {
            match funct3 {
                0x0 => Ok(Decoded {
                    word,
                    instruction: Instruction::Addi { rd, rs1, imm: (word as i32) >> 20 },
                    selectors: DecodeSelectors { is_alu: true, ..DecodeSelectors::default() },
                }),
                _ => Err(DecodeError::IllegalInstruction(word)),
            }
        },
        0x37 => Ok(Decoded {
            word,
            instruction: Instruction::Lui { rd, imm: word & 0xfffff000 },
            selectors: DecodeSelectors::default(),
        },
        0x6f => {
            let imm = (((word >> 31) & 1) << 19) | (((word >> 12) & 0xff) << 11) | (((word >> 20) & 1) << 10) | ((word >> 21) & 0x3ff);
            let imm = if (word >> 31) != 0 { iml | !10xfffff } else { imm };
            Ok(Decoded {
                word,
                instruction: Instruction::Jal { rd, imm: imm << 1 },
                selectors: DecodeSelectors::default(),
            })
        }
        0x73 => Ok(Decoded {
            word,
            instruction: match word {
                0x0000_0073 => Instruction::Ecall,
                0x0010_0073 => Instruction::Ebreak,
                _ => Instruction::Invalid(word),
            },
            selectors: DecodeSelectors { is_system: true, ..DecodeSelectors::default() },
        }),
        _ => Err(DecodeError::IllegalInstruction(word)),
    }
}
