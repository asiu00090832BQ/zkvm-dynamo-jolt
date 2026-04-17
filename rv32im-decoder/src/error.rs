use core::fmt;

[pub enum DecoderError {
    InvalidOpcode(u8),
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct3: u8, funct7: u8 },
    InvalidSystemImm(u16),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoderError::InvalidOpcode(opcode) => {
                write!(f, "invalid opcode: 0b{:width_bits}b, opcode)
            }
            DecoderError::InvalidFunct3 { opcode, funct3 } => {
                write!(f, "invalid funct3: opcode=0b{opcode:07b}, funct3=0b{funct3:03b}")
            }
            DecoderError::InvalidFunct7 {
                opcode,
                funct3,
                funct7,
            } => {
                write!(f, "invalid funct7: opcode=0b{opcode:07b}, funct3=0b{funct3:03b}, funct7=0b{funct7:07b}")
            }
            DecoderError::InvalidSystemImm(imm) => {
                write!(f, "invalid system immediate: 0x{imm:03x}")
            }
        }
    }
}

impl core::error::Error for DecoderError {}
