use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: usize, rs1: usize, rs2: usize },
    Sub { rd: usize, rs1: usize, rs2: usize },
    Ecall,
    Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidInstruction(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    match word {
        0x00000073 => return Ok(Instruction::Ecall),
        0x00100073 => return Ok(Instruction::Ebreak),
        _ => {}
    }

    let opcode = word & 0x7f;
    if opcode == 0x33 {
        let rd = ((word >> 7) & 0x1f) as usize;
        let funct3 = (word >> 12) & 0x7;
        let rs1 = ((word >> 15) & 0x1f) as usize;
        let rs2 = ((word >> 20) & 0x1f) as usize;
        let funct7 = (word >> 25) & 0x7f;

        return match (funct7, funct3) {
            (0x00, 0x0) => Ok(Instruction::Add { rd, rs1, rs2 }),
            (0x20, 0x0) => Ok(Instruction::Sub { rd, rs1, rs2 }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        };
    }

    Err(DecodeError::InvalidInstruction(word))
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add { rd, rs1, rs2 } => write!(f, "add x{}, x{}, x{}", rd, rs1, rs2),
            Instruction::Sub { rd, rs1, rs2 } => write!(f, "sub x{}, x{}, x{}", rd, rs1, rs2),
            Instruction::Ecall => f.write_str("ecall"),
            Instruction::Ebreak => f.write_str("ebreak"),
        }
    }
}
