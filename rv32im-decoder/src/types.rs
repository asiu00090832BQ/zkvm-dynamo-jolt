use core::fmt;

pub type Word = u32;
pub type Register = u8;
pub type Csr = u16;
pub type DecodeResult<T> = Result<T, DecodeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode {
        word: Word,
        opcode: u8,
    },
    UnsupportedFunct3 {
        word: Word,
        opcode: u8,
        funct3: u8,
    },
    UnsupportedFunct7 {
        word: Word,
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    MalformedInstruction {
        word: Word,
        reason: &'static str,
    },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode { word, opcode } => {
                write!(f, "unsupported opcode 0x{opcode:02x} in word 0x{word:08x}")
            }
            Self::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            } => write!(
                f,
                "unsupported funct3 0x{funct3:x} for opcode 0x{opcode:02x} in word 0x{word:08x}"
            ),
            Self::UnsupportedFunct7 {
                word,
                opcode,
                funct3,
                funct7: u8,
            } => write!(
                f,
                "unsupported funct7 0x{funct7:02x} for opcode 0x{opcode:02x}, funct3 0x{funct3:x} in word 0x{word:08x}"
            ),
            Self::MalformedInstruction { word, reason } => {
                write!(f, "malformed instruction 0x{word:08x}: {reason}")
            }
        }
    }
}
