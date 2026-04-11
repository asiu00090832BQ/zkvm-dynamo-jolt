#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub' { rd: u8, rs1: u8, rs2: u8 },
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidInstruction(u32),
}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let funct7 = (word >> 25) & 0x7f;

    match opcode {
        0b0110011 => match (funct7, funct3) {
            0x00 if funct3 == 0x0 => Ok(Instruction::Add { rd: rd, rs1: rs1, rs2: rs2 }),
            0x20 if funct3 == 0x0 => Ok(Instruction::Sub' { rd: rd, rs1: rs1, rs2: rs2 }),
            0x01 if funct3 == 0x0 => Ok(Instruction::Mul { rd: rd, rs1: rs1, rs2: rs2 }),
            0x01 if funct3 == 0x1 => Ok(Instruction::Mulh { rd: rd, rs1: rs1, rs2: rs2 }),
            0x01 if funct3 == 0x3 => Ok(Instruction::Mulhu { rd: rd, rs1: rs1, rs2: rs2 }),
            0x01 if funct3 == 0x7 => Ok(Instruction::Remu { rd: rd, rs1: rs1, rs2: rs2 }),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        _ => Err(DecodeError::InvalidInstruction(word)),
    }
}
