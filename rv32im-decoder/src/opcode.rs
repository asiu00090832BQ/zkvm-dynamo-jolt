use crate::error::DecodeError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Load = 0b0000011,
    MiscMem = 0b0001111,
    OpImm = 0b0010011,
    Auipc = 0b0010111,
    Store = 0b0100011,
    Op = 0b0110011,
    Lui = 0b0110111,
    Branch = 0b1100011,
    Jalr = 0b1100111,
    Jal = 0b1101111,
    System = 0b1110011,
}

impl Opcode {
    pub const fn bits(self) -> u8 {
        self as u8
    }

    pub fn from_word(word: u32) -> Result<Self, DecodeError> {
        Self::from_bits((word & 0x7f) as u8)
    }

    pub fn from_bits(bits: u8) -> Result<Self, DecodeError> {
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
            _ => Err(DecodeError::InvalidOpcode(bits)),
        }
    }
}
