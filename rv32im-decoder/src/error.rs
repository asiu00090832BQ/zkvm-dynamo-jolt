use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    UnsupportedOpcode(u8),
    UnsupportedInstruction { opcode: u8, funct3: u8, funct7: u8 },
    InvalidShiftEncoding { funct7: u8 },
    InvalidSystemInstruction { funct3: u8, imm12: u16 },
    ReservedEncoding(&'static str),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0b{opcode:07b}"),
            Self::UnsupportedInstruction { opcode, funct3, funct7 } => write!(f, "unsupported instruction: opcode=0b{opcode:07b}, funct3=0b{funct3:03b}, funct7=0b{funct7:07b}"),
            Self::InvalidShiftEncoding { funct7 } => write!(f, "invalid shift immediate encoding: funct7=0b{funct7:07b}"),
            Self::InvalidSystemInstruction { funct3, imm12 } => write!(f, "invalid system instruction: funct3=0b{funct3:03b}, imm12=0x{imm12:03x}"),
            Self::ReservedEncoding(message) => write!(f, "reserved encoding: {message}"),
        }
    }
}
