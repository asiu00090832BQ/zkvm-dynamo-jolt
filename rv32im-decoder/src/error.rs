use core::fmt;

pub type Result<T> = core::result::Result<T, ZkvmError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZkvmError {
    UnsupportedCompressedInstruction { halfword: u16 },
    UnknownOpcode { opcode: u8, word: u32 },
    UnknownFunct3 { opcode: u8, funct3: u8, word: u32 },
    UnknownFunct7 {
        opcode: u8,
        funct3: u8,
        funct7: u8,
        word: u32,
    },
    InvalidRegister { reg: u8 },
    InvalidShiftEncoding { funct7: u8, word: u32 },
    InvalidFenceEncoding { word: u32 },
    InvalidSystemEncoding { imm12: u16, word: u32 },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedCompressedInstruction { halfword } => {
                write!(
                    f,
                    "unsupported compressed or non-32-bit instruction: 0x{halfword:04x}"
                )
            }
            Self::UnknownOpcode { opcode, word } => {
                write!(f, "unknown opcode 0b{opcode:07b} in instruction 0x{word:08x}")
            }
            Self::UnknownFunct3 {
                opcode,
                funct3,
                word,
            } => write!(
                f,
                "unknown funct3 0b{funct3:03b} for opcode 0b{opcode:07b} in instruction 0x{word:08x}"
            ),
            Self::UnknownFunct7 {
                opcode,
                funct3,
                funct7,
                word,
            } => write!(
                f,
                "unknown funct7 0b{funct7:07b} for opcode 0b{opcode:07b}, funct3 0b{funct3:03b} in instruction 0x{word:08x}"
            ),
            Self::InvalidRegister { reg } => {
                write!(f, "invalid RV32 register index: {reg}")
            }
            Self::InvalidShiftEncoding { funct7, word } => write!(
                f,
                "invalid RV32I shift-immediate encoding with funct7 0b{funct7:07b} in instruction 0x{word:08x}"
            ),
            Self::InvalidFenceEncoding { word } => {
                write!(f, "invalid FENCE encoding in instruction 0x{word:08x}")
            }
            Self::InvalidSystemEncoding { imm12, word } => write!(
                f,
                "invalid SYSTEM encoding with imm12 0x{imm12:03x} in instruction 0x{word:08x}"
            ),
        }
    }
}
