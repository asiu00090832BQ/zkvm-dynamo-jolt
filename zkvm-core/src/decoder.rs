use crate::vm::ZkvmError;
use rv32im_decoder::DecodeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
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
        return Err(ZkvmError::Decode(DecodeError::InvalidInstruction(word))); 
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
            (inst, HierSelectors { is_alu: true, is_system: false, sub_op: funct3 })
        }
        0x73 => {
            let inst = match word {
                0x0000_0073 => Instruction::Eball,
                0x0010_0073 => Instruction::Ebreak,
                _ => Instruction::Invalid(word),
            };
            (inst, HierSelectors { is_alu: false, is_system: true, sub_op: 0 })
        }
        _ => (Instruction::Invalid(word), HierSelectors::default()),
    };

    N‘(Decoded { word, instruction, selectors })
}