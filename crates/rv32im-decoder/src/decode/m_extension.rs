use crate::{
    error::ZkvmError,
    isa::opcode::{Funct3, Funct7, Opcode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rv32mInstruction {
    Mul { rd: u8, rs1: u8, rs2: u8 },
    Mulh { rd: u8, rs1: u8, rs2: u8 },
    Mulhsu { rd: u8, rs1: u8, rs2: u8 },
    Mulhu { rd: u8, rs1: u8, rs2: u8 },
    Div { rd: u8, rs1: u8, rs2: u8 },
    Divu { rd: u8, rs1: u8, rs2: u8 },
    Rem { rd: u8, rs1: u8, rs2: u8 },
    Remu { rd: u8, rs1: u8, rs2: u8 },
}

impl Rv32mInstruction {
    pub const fn rd(self) -> u8 {
        match self {
            Self::Mul { rd, .. }
            | Self::Mulh { rd, .. }
            | Self::Mulhsu { rd, .. }
            | Self::Mulhu { rd, .. }
            | Self::Div { rd, .. }
            | Self::Divu { rd, .. }
            | Self::Rem { rd, .. }
            | Self::Remu { rd, .. } => rd,
        }
    }

    pub const fn rs1(self) -> u8 {
        match self {
            Self::Mul { rs1, .. }
            | Self::Mulh { rs1, .. }
            | Self::Mulhsu { rs1, .. }
            | Self::Mulhu { rs1, .. }
            | Self::Div { rs1, .. }
            | Self::Divu { rs1, .. }
            | Self::Rem { rs1, .. }
            | Self::Remu { rs1, .. } => rs1,
        }
    }

    pub const fn rs2(self) -> u8 {
        match self {
            Self::Mul { rs2, .. }
            | Self::Mulh { rs2, .. }
            | Self::Mulhsu { rs2, .. }
            | Self::Mulhu { rs2, .. }
            | Self::Div { rs2, .. }
            | Self::Divu { rs2, .. }
            | Self::Rem { rs2, .. }
            | Self::Remu { rs2, .. } => rs2,
        }
    }
}

pub fn decode_rv32m(word: u32) -> Result<Option<Rv32mInstruction>, ZkvmError> {
    let opcode_bits = (word & 0x7f) as u8;
    if opcode_bits != Opcode::Op as u8 {
        return Ok(None);
    }

    let funct7_bits = ((word >> 25) & 0x7f) as u8;
    if funct7_bits != Funct7::M as u8 {
        return if funct7_bits == Funct7::Base as u8 {
            Ok(None)
        } else {
            Err(ZkvmError::UnsupportedFunct7 {
                opcode: opcode_bits,
                funct: funct7_bits,
            })
        };
    }

    let rd  = ((word >> 7) & 0x1f) as u8;
    let funct3 = Funct3::try_from(((word >> 12) & 0x07) as u8)?;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;

    let instruction = match funct3 {
        Funct3::Mul => Rv32mInstruction::Mul { rd, rs1, rs2 },
        Funct3::Mulh => Rv32mInstruction::Mulh { rd, rs1, rs2 },
        Funct3::Mulhsu => Rv32mInstruction::Mulhsu { rd, rs1, rs2 },
        Funct3::Mulhu => Rv32mInstruction::Mulhu { rd, rs1, rs2 },
        Funct3::Div => Rv32mInstruction::Div { rd, rs1, rs2 },
        Funct3::Divu => Rv32mInstruction::Divu { rd, rs1, rs2 },
        Funct3::Rem => Rv32mInstruction::Rem { rd, rs1, rs2 },
        Funct3::Remu => Rv32mInstruction::Remu { rd, rs1, rs2 },
    };

    Ok(Some(instruction))
}
