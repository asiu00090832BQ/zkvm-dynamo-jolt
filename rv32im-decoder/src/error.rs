use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode(u32),
    InvalidInstruction(u32),
    InvalidFunct3 { word: u32, opcode: u32, funct3: u32 },
    InvalidFunct7 { word: u32, opcode: u32, funct3: u32, funct7: u32 },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode(opcode) => write!(f, "invalid opcode: 0b{opcode:07b}"),
            Self::InvalidInstruction(word) => write!(f, "invalid instruction: 0x{word:08x}"),
            Self::InvalidFunct3 { word, opcode, funct3 } => {
                write!(
                    f,
                    "invalid funct3 for opcode 0b{opcode:07b}: word=0x{word:08x}, funct3=0b{funct3:03b}"
                )
            }
            Self::InvalidFunct7 {
                word,
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "invalid funct7 for opcode 0b{opcode:07b}, funct3=0b{funct3:03b}: word=0x{word:08x}, funct7=0b{funct7:07b}"
                )
            }
        }
    }
}
