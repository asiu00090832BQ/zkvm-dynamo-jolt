use core::fmt;

use crate::error::DecodeError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Opcode {
    Load,
    MiscMem,
    OpImm,
    Auipc,
    Store,
    Op,
    Lui,
    Branch,
    Jalr,
    Jal,
    System,
}

impl Opcode {
    pub const fn bits(self) -> u8 {
        match self {
            Self::Load => 0b0000011,
            Self::MiscMem => 0b0001111,
            Self::OpImm => 0b0010011,
            Self::Auipc => 0b0010111,
            Self::Store => 0b0100011,
            Self::Op => 0b0110011,
            Self::Lui => 0b0110111,
            Self::Branch => 0b1100011,
            Self::Jalr => 0b1100111,
            Self::Jal => 0b1101111,
            Self::System => 0b1110011,
        }
    }

    pub fn decode(bits: u8) -> Result<Self, DecodeError> {
        match bits {
            0b0000011 => Ok(Self::Load),
            0b0001111 => Ok(Self::MiscMem),
            0b0010011 => Ok(Self::OpImm),
            0b0010111 => Ok(Self::Auipc),
            0b0100011 => Ok(Self::Store),
            0b0110011 => Ok(Self::Op),
            0b0110111 => Ok(Self::Lui),
            0b1100011 => Ok(Self::Branch),
            0b1100111 => Ok(Self::Jalr),
            0b1101111 => Ok(Self::Jal),
            0b1110011 => Ok(Self::System),
            other => Err(DecodeError::UnsupportedOpcode(other)),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Load => "LOAD",
            Self::MiscMem => "MISC-MEM",
            Self::OpImm => "OP-IMM",
            Self::Auipc => "AUIPC",
            Self::Store => "STORE",
            Self::Op => "OP",
            Self::Lui => "LUI",
            Self::Branch => "BRANCH",
            Self::Jalr => "JALR",
            Self::Jal => "JAL",
            Self::System => "SYSTEM",
        };
        f.write_str(name)
    }
}
