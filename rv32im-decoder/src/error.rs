use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, ZkvmError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidEncoding { word: u32, reason: &'static str },
    UnsupportedOpcode { opcode: u8, word: u32 },
    UnsupportedFunct3 { funct3: u8, opcode: u8, word: u32 },
    UnsupportedFunct7 { funct7: u8, funct3: u8, opcode: u8, word: u32 },
    InvalidRegister { index: u8 },
    ParseError(String),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::InvalidEncoding { word, reason } => {
                write!(f, "invalid instruction encoding {word:#010x}: {reason}")
            }
            ZkvmError::UnsupportedOpcode { opcode, word } => {
                write!(f, "unsupported opcode {opcode:#04x} in word {word:#010x}")
            }
            ZkvmError::UnsupportedFunct3 {
                funct3,
                opcode,
                word,
            } => {
                write!(
                    f,
                    "unsupported funct3 {funct3:#03b} for opcode {opcode:#04x} in word {word:#010x}"
                )
            }
            ZkvmError::UnsupportedFunct7 {
                funct7,
                funct3,
                opcode,
                word,
            } => {
                write!(
                    f,
                    "unsupported funct7 {funct7:#09b} with funct3 {funct3:#03b} for opcode {opcode:#04x} in word {word:#010x}"
                )
            }
            ZkvmError::InvalidRegister { index } => {
                write!(f, "invalid register index x{index}")
            }
            ZkvmError::ParseError(message) => f.write_str(message),
        }
    }
}

impl Error for ZkvmError {}
