use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode(u8),
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct7: u8 },
    InvalidRegister(u8),
    UnsupportedInstruction(u32),
    MalformedImmediate(&'static str),
    InvariantViolation(&'static str),
    Arithmetic(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode: 0b{opcode:07b}"),
            Self::InvalidFunct3 { opcode, funct3 } => {
                write!(f, "invalid funct3 0b{funct3:03b} for opcode 0b{opcode:07b}")
            }
            Self::InvalidFunct7 { opcode, funct7 } => {
                write!(f, "invalid funct7 0b{funct7:07b} for opcode 0b{opcode:07b}")
            }
            Self::InvalidRegister(register) => write!(f, "invalid register index: {register}"),
            Self::UnsupportedInstruction(word) => write!(f, "unsupported instruction word: 0x{word:08x}"),
            Self::MalformedImmediate(label) => write!(f, "malformed immediate: {label}"),
            Self::InvariantViolation(label) => write!(f, "invariant violation: {label}"),
            Self::Arithmetic(label) => write!(f, "arithmetic error: {label}"),
        }
    }
}

impl Error for DecodeError {}
