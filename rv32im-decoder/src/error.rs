use core::fmt;

pub type Result<T> = core::result::Result<T, DecodeError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u8),
    UnsupportedFunct3 { opcode: u8, funct3: u8 },
    UnsupportedFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    InvalidShiftEncoding { funct7: u8 },
    InvalidSystemEncoding(u32),
    InvariantViolation(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode(opcode) => {
                write!(f, "unsupported opcode 0b{opcode:07b}")
            }
            Self::UnsupportedFunct3 { opcode, funct3 } => write!(
                f,
                "unsupported funct3 0b{funct3:03b} for opcode 0b{opcode:07b}"
            ),
            Self::UnsupportedFunct7 {
                opcode,
                funct3,
                funct7,
            } => write!(
                f,
                "unsupported funct7 0b{funct7:07b} for opcode 0b{opcode:07b} / funct3 0b{funct3:03b}"
            ),
            Self::InvalidShiftEncoding { funct7 } => {
                write!(f, "invalid shift immediate encoding with funct7 0b{funct7:07b}")
            }
            Self::InvalidSystemEncoding(word) => {
                write!(f, "invalid system instruction encoding 0x{word:08x}")
            }
            Self::InvariantViolation(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for DecodeError {}
