use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedCompressed(u16),
    InvalidOpcode(u8),
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct7: u8 },
    InvalidSystem(u32),
    IllegalInstruction(u32),
    Validation(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCompressed(bits) => {
                write!(f, "compressed/16-bit instruction not supported: 0x{bits:04x}")
            }
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode: 0b{opcode:07b}"),
            Self::InvalidFunct3 { opcode, funct3 } => {
                write!(f, "invalid funct3 0b{funct3:03b} for opcode 0b{opcode:07b}")
            }
            Self::InvalidFunct7 { opcode, funct7 } => {
                write!(f, "invalid funct7 0b{funct7:07b} for opcode 0b{opcode:07b}")
            }
            Self::InvalidSystem(word) => write!(f, "invalid system instruction: 0x{word:08x}"),
            Self::IllegalInstruction(word) => write!(f, "illegal instruction: 0x{word:08x}"),
            Self::Validation(msg) => f.write_str(msg),
        }
    }
}

impl std::error::Error for DecodeError {}
