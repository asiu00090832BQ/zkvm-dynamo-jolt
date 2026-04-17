use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    InvalidRegister(u8),
    UnsupportedOpcode(u8),
    UnsupportedFunct3 { opcode: u8, funct3: u8 },
    UnsupportedFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    UnsupportedShiftEncoding { funct3: u8, funct7: u8 },
    UnsupportedSystem(u32),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction(word) => write!(f, "invalid instruction encoding: 0x{word:08x}"),
            Self::InvalidRegister(index) => write!(f, "invalid register index: x{index}"),
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0b{opcode:07b}"),
            Self::UnsupportedFunct3 { opcode, funct3 } => {
                write!(f, "unsupported funct3=0b{funct3:03b} for opcode=0b{opcode:07b}")
            }
            Self::UnsupportedFunct7 {
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "unsupported funct7=0b{funct7:07b} for opcode=0b{opcode:07b}, funct3=0b{funct3:03b}"
                )
            }
            Self::UnsupportedShiftEncoding { funct3, funct7 } => {
                write!(
                    f,
                    "unsupported shift encoding: funct3=0b{funct3:03b}, funct7=0b{funct7:07b}"
                )
            }
            Self::UnsupportedSystem(word) => write!(f, "unsupported system instruction: 0x{word:08x}"),
        }
    }
}
