use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Non32BitInstruction { word: u32 },
    UnsupportedOpcode { word: u32, opcode: u8 },
    UnsupportedFunct3 { word: u32, opcode: u8, funct3: u8 },
    UnsupportedFunct7 {
        word: u32,
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    UnsupportedSystem { word: u32, funct3: u8, imm12: u16 },
}

pub type ZkvmError = DecodeError;

impl DecodeError {
    #[inline]
    pub const fn into_zkvm_error(self) -> ZkvmError {
        self
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Non32BitInstruction { word } => {
                write!(f, "non-32-bit instruction encoding: 0x{word:08x}")
            }
            Self::UnsupportedOpcode { word, opcode } => {
                write!(f, "unsupported opcode 0b{opcode:07b} in 0x{word:08x}")
            }
            Self::UnsupportedFunct3 {
                word,
                opcode,
                funct3,
            } => {
                write!(
                    f,
                    "unsupported funct3 0b{funct3:03b} for opcode 0b{opcode:07b} in 0x{word:08x}"
                )
            }
            Self::UnsupportedFunct7 {
                word,
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "unsupported funct7 0b{funct7:07b} for opcode 0b{opcode:07b} and funct3 0b{funct3:03b} in 0x{word:08x}"
                )
            }
            Self::UnsupportedSystem { word, funct3, imm12 } => {
                write!(
                    f,
                    "unsupported SYSTEM encoding funct3=0b{funct3:03b}, imm12=0x{imm12:03x} in 0x{word:08x}"
               )
            }
        }
    }
}
