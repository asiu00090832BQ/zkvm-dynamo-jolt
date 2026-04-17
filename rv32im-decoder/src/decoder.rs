use crate::instruction::Instruction;
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u8),
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    ReservedEncoding(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode 0b{opcode:07b}"),
            Self::InvalidFunct3 { opcode, funct3 } => {
                write!(
                    f,
                    "invalid funct3 0b{funct3:03b} for opcode 0b{opcode:07b}"
                )
            }
            Self::InvalidFunct7 {
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "invalid funct7 0b{funct7:07b} for opcode 0b{opcode:07b}, funct3 0b{funct3:03b}"
                )
            }
            Self::ReservedEncoding(word) => write!(f, "reserved or illegal encoding 0x{word:08x}"),
        }
    }
}

impl std::error::Error for DecodeError {}

#[inline]
pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    crate::decode::decode_word(word)
}
