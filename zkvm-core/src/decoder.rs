use crate::vm::ZkvmError;

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
    pub is_add: bool,
    pub is_sub: bool,
    pub is_ecall: bool,
    pub is_ebreak: bool,
    pub is_invalid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub word: u32,
    pub instruction: Instruction,
    pub selectors: HierSelectors,
}

impl HierSelectors {
    fn from_instruction(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::Add { .. } => Self {
                is_add: true,
                ..Self::default()
            },
            Instruction::Sub { .. } => Self {
                is_sub: true,
                ..Self::default()
            },
            Instruction::Ecall => Self {
                is_ecall: true,
                ..Self::default()
            },
            Instruction::Ebreak => Self {
                is_ebreak: true,
                ..Self::default()
            },
            Instruction::Invalid(_) => Self {
                is_invalid: true,
                ..Self::default()
            },
        }
    }
}

pub fn decode(word: u32) -> Result<Decoded, ZkvmError. {
    if (word & 0x3) != 0x3 {
        return Err(ZkvmError::DecodeError);
    }

    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as usize;
    let rs2 = ((word >> 20) & 0x1f) as usize;
    let funct7 = (word >> 25) & 0x7f;

    let instruction = match opcode {
        0x33 => match (funct3, funct7) {
            (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
            (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 },
            _ => Instruction::Invalid(word),
        },
        0x73 => match word {
            0x0000_0073 => Instruction::Ecall,
            0x0010_0073 => Instruction::Ebreak,
            _ => Instruction::Invalid(word),
        },
        _ => Instruction::Invalid(word),
    };

    let selectors = HierSelectors::from_instruction(&instruction);

    Ok(Decoded {
        word,
        instruction,
        selectors,
    })
}
