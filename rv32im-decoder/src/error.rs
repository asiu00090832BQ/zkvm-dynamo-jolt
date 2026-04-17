use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    CompressedInstructionUnsupported {
        word: u32,
    },
    InvalidOpcode {
        opcode: u8,
        word: u32,
    },
    InvalidFunct3 {
        funct3: u8,
        opcode: u8,
        word: u32,
    },
    InvalidFunct7 {
        funct7: u8,
        opcode: u8,
        word: u32,
    },
    InvalidRegister {
        reg: u8,
        context: &'static str,
    },
    UnsupportedInstruction {
        word: u32,
        reason: &'static str,
    },
    DecodeInvariantViolation {
        word: u32,
        message: &'static str,
    },
    ParseError {
        input: String,
    },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CompressedInstructionUnsupported { word } => {
                write!(f, "compressed or non-32-bit instruction unsupported: 0x{word:08x}")
            }
            Self::InvalidOpcode { opcode, word } => {
                write!(f, "invalid opcode 0b{opcode:07b} for word 0x{word:08x}")
            }
            Self::InvalidFunct3 { funct3, opcode, word } => {
                write!(
                    f,
                    "invalid funct3 0b{funct3:03b} for opcode 0b{opcode:07b} in word 0x{word:08x}"
                )
            }
            Self::InvalidFunct7 { funct7, opcode, word } => {
                write!(
                    f,
                    "invalid funct7 0b{funct7:07b} for opcode 0b{opcode:07b} in word 0x{word:08x}"
                )
            }
            Self::InvalidRegister { reg, context } => {
                write!(f, "invalid register {reg} in {context}")
            }
            Self::UnsupportedInstruction { word, reason } => {
                write!(f, "unsupported instruction 0x{word:08x}: {reason}")
            }
            Self::DecodeInvariantViolation { word, message } => {
                write!(f, "decode invariant violation for 0x{word:08x}: {message}")
            }
            Self::ParseError { input } => {
                write!(f, "failed to parse instruction word from input: {input}")
            }
        }
    }
}

impl std::error::Error for ZkvmError {}
