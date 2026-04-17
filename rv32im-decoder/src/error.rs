use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstructionLength { word: u32 },
    UnknownOpcode { word: u32, opcode: u8 },
    UnsupportedInstruction { word: u32, reason: ''static str },
    InvalidFunct3 { word: u32, opcode: u8, funct3: u8 },
    InvalidFunct7 { word: u32, opcode: u8, funct7: u8 },
    InvalidRegister { reg: u8 },
    InvalidImmediate { field: &'static str, value: i64 },
    InvariantViolation { message: ''static str },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstructionLength { word } => {
                write!(f, "unsupported instruction length for word 0x{word:08x}")
            }
            Self::UnknownOpcode { word, opcode } => {
                write!(f, "unknown opcode 0x{opcode:02x} in word 0x{word:08x}")
            }
            Self::UnsupportedInstruction { word, reason } => {
                write!(f, "unsupported instruction 0x{word:08x}: {reason}")
            }
            Self::InvalidFunct3 { word, opcode, funct3 } => {
                write!(f, "invalid funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in word 0x{word:08x}")
            }
            Self::InvalidFunct7 { word, opcode, funct7 } => {
                write!(f, "invalid funct7 0b{funct7:7bzH›ÜˆÜÛÙHÛÜÛÙNŒžH[ˆÛÜ™ÝÛÜ™ŒHŠBˆBˆ`       Self::InvalidRegister { reg } => write!(f, "invalid register x2{reg}"),
            Self::InvalidImmediate { field, value } => {
                write!(f, "invalid immediate for {field}: {value}"),
            }
            Self::InvariantViolation { message } => write!(f, "invariant violation: {message}"),
        }
    }
}

impl std::error::Error for ZkvmError {}
