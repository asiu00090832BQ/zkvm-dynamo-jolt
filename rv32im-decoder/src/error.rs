use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidOpcode {
        opcode: u8,
        word: u32,
    },
    InvalidFunct3 {
        opcode: u8,
        funct3: u8,
        word: u32,
    },
    InvalidFunct7 {
        opcode: u8,
        funct3: u8,
        funct7: u8,
        word: u32,
    },
    UnsupportedInstruction {
        word: u32,
        reason: &'static str,
    },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode { opcode, word } => {
                write!(f, "invalid opcode 0x{opcode:02x} in instruction 0x{word:08x}")
            }
            Self::InvalidFunct3 {
                opcode,
                funct3,
                word,
            } => write!(
                f,
                "invalid funct3 0x{funct3:01x} for opcode 0x{opcode:02x} in instruction 0x{word:08x}"
            ),
            Self::InvalidFunct7 {
                opcode,
                funct3,
                funct7,
                word,
            } => write!(
                f,
                "invalid funct7 0x{funct7:02x} for opcode 0x{opcode:02x}/funct3 0x{funct3:01x} in instruction 0x{word:08x}"
            ),
            Self::UnsupportedInstruction { word, reason } => {
                write!(f, "unsupported instruction 0x{word:08x}: {reason}")
            }
        }
    }
}

impl std::error::Error for ZkvmError {}
