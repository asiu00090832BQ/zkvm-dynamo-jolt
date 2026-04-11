use std::fmt::{Display, Formatter};

use crate::vm::ZkvmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidInstruction(u32),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInstruction(word) => {
                write!(f, "invalid instruction 0x{word:08x}")
            }
        }
    }
}

impl std::error::Error for DecodeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add,
    Sub,
    Ecall,
    Ebreak,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HierSelectors {
    pub is_alu: bool,
    pub is_system: bool,
    pub sub_op: bool,
}

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
    let opcode = (word & 0x7f) as u8;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = ((word >> 12) & 0x07) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = ((word >> 25) & 0x7f) as u8;

    let decoded = match opcode {
        0x33 => match (funct3, funct7) {
            (0x0, 0x00) => Decoded {
                Ý½É°(€€€€€€€€€€€€€€€¥¹ÍÑÉÕÑ¥½¸è%¹ÍÑÉÕÑ¥½¸èé‘°(€€€€€€€€€€€€€€€É°(€€€€€€€€€€€€€€€ÉÌÄ°(€€€€€€€€€€€€€€€ÉÌÈ°(€€€€€€€€€€€€€€€™Õ¹ÐÌ°(€€€€€€€€€€€€€€€™Õ¹ÐÜ°(€€€€€€€€€€€€€€€Í•±•Ñ½ÉÌè!¥•ÉM•±•Ñ½ÉÌì(€€€€€€€€€€€€€€€€€€€¥Í}…±ÔèÑÉÕ”°(€€€€€€€€€€€€€€€€€€€¥Í}ÍåÍÑ•´è™…±Í”°(€€€€€€€€€€€€€€€€€€€sub_op: false,
                },
            },
            (0x0, 0x20) => Decoded {
                word,
                instruction: Instruction::Sub,
                rd,
                rs1,
                rs2,
                funct3,
                funct7,
                selectors: HierSelectors {
                    is_alu: true,
                    is_system: false,
                    sub_op: true,
                },
            },
            _ => {
                return Err(ZkvmError::Decode(DecodeError::InvalidInstruction(word)));
            }
        },
        0x73 => match word {
            0x00000073 => Decoded {
                word,
                instruction: Instruction::Ecall,
                rd,
                rs1,
               rs2,
                funct3,
                funct7,
                selectors: HierSelectors {
                    is_alu: false,
                    is_system: true,
                    sub_op: false,
                },
            },
            0x00100073 => Decoded {
                word,
                instruction: Instruction::Ebreak,
                rd,
                rs1,
                rs2,
                funct3,
                funct7,
                selectors: HierSelectors {
                    is_alu: false,
                    is_system: true,
                    sub_op: false,
                },
            },
            _ => {
                return Err(ZkvmError::Decode(DecodeError::InvalidInstruction(word)));
            }
        },
        _ => {
            return Err(ZkvmError::Decode(DecodeError::InvalidInstruction(word)));
        }
    }:

    Ok(decoded)
}