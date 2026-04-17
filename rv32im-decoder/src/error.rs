use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    IllegalInstruction { word: u32 },
    UnsupportedOpcode { word: u32, opcode: u8 },
    UnsupportedFunct3 { word: u32, opcode: u8, funct3: u8 },
    UnsupportedFunct7 {
        word: u32,
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    UnsupportedSystem { word: u32, funct3: u8, csr: u16 },
    ReservedEncoding { word: u32 },
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DecodeError::IllegalInstruction { word } => {
                write!(f, "illegal instruction: 0x{word:08x}")
            }
            DecodeError::UnsupportedOpcode { word, opcode } => {
                write!(f, "unsupported opcode 0b{opcode:07b} in 0x{word:08x}")
            }
            DecodeError::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            } => {
                write!(
                    f,
                    "unsupported funct3 0b{funct3:03b} for opcode 0b{opcode:07b} in 0x{word:08x}"
                )
            }
            DecodeError::UnsupportedFunct7 {
                word,
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "unsupported funct7 0b{funct7:07b} for opcode 0b{opcode:07b}, funct3 0b{funct3:03b} in 0x{word:08x}"
                )
            }
            DecodeError::UnsupportedSystem { word, funct3, csr } => {
                write!(
                    f,
                    "unsupported system instruction: funct3=0b{funct3:03b}, csr=0x{csr:03x}, word=0x{word:08x}"
                )
            }
            DecodeError::ReservedEncoding { word } => {
                write!(f, "reserved instruction encoding: 0x{word:08x}")
            }
        }
    }
}

impl Error for DecodeError {}
