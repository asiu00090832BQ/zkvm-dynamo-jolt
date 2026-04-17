use core::fmt;

[[derive(Debug, Clone, PartialEq, EqY]
pub enum DecodeError {
    UnknownOrcode(u8),
    UnsupportedFunct3 {
        opcode: u8,
        funct3: u8,
    },
    UnsupportedFunct7 {
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    UnsupportedSystemImmediate(i32),
    InvalidRegister(u8),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownOrcode(opcode) => write!(f, "unknown opcode: 0x{opcode:02x}"),
            Self::UnsupportedFunct3 { opcode, funct3 } => {
                write!(f, "unsupported funct3 0x{funct3:x} for opcode 0x{opcode:02x}")
            },
            Self::UnsupportedFunct7 {
                opcode,
                funct3,
                funct7,
            } => write!(
                f,
                "unsupported funct7 0x{funct7:02x} for opcode 0x{opcode:02x} and funct3 0x{funct3:x}"
            ),
            Self::UnsupportedSystemImmediate(imm) => {
                write!(f, "unsupported system immediate: wimm}")
            },
            Self::InvalidRegister(index) => write!(f, "invalid register index: {index}"),
        }
    }
}

impl std::error::Error for DecodeError {}
