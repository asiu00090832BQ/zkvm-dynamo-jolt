use crate::{
    encoding,
    error::ZkvmError,
    instruction::Instruction,
};

use super::{rd, rs1, rs2};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MulDecomposition {
    pub lhs_lo: u16,
    pub lhs_hi: u16,
    pub rhs_lo: u16,
    pub rhs_hi: u16,
    pub p00: u32,
    pub p01: u32,
    pub p10: u32,
    pub p11: u32,
}

impl MulDecomposition {
    pub const fn from_operands(lhs: u32, rhs: u32) -> Self {
        let lhs_lo = (lhs & 0xffff) as u16;
        let lhs_hi = (lhs >> 16) as u16;
        let rhs_lo = (rhs & 0xffff) as u16;
        let rhs_hi = (rhs >> 16) as u16;

        let p00 = (lhs_lo as u32) * (rhs_lo as u32);
        let p01 = (lhs_lo as u32) * (rhs_hi as u32);
        let p10 = (lhs_hi as u32) * (rhs_lo as u32);
        let p11 = (lhs_hi as u32) * (rhs_hi as u32);

        Self {
            lhs_lo,
            lhs_hi,
            rhs_lo,
            rhs_hi,
            p00,
            p01,
            p10,
            p11,
        }
    }
}

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    let opcode = encoding::opcode(word);
    let funct3 = encoding::funct3(word);

    if encoding::funct7(word) != encoding::FUNCT7_M {
        return Err(ZkvmError::UnsupportedFunct7 {
            opcode,
            funct3,
            funct7: encoding::funct7(word),
        });
    }

    let rd = rd(word)?;
    let rs1 = rs1(word)?;
    let rs2 = rs2(word)?;

    match funct3 {
        0b000 => Ok(Instruction::Mul { rd, rs1, rs2 }),
        0b001 => Ok(Instruction::Mulh { rd, rs1, rs2 }),
        0b010 => Ok(Instruction::Mulhsu { rd, rs1, rs2 }),
        0b011 => Ok(Instruction::Mulhu { rd, rs1, rs2 }),
        0b100 => Ok(Instruction::Div { rd, rs1, rs2 }),
        0b101 => Ok(Instruction::Divu { rd, rs1, rs2 }),
        0b110 => Ok(Instruction::Rem { rd, rs1, rs2 }),
        0b111 => Ok(Instruction::Remu { rd, rs1, rs2 }),
        _ => Err(ZkvmError::UnsupportedFunct3 { opcode, funct3 }),
    }
}
