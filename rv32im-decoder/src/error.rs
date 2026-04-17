use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    UnsupportedOpcode { opcode: u8 },
    UnsupportedInstruction { opcode: u8, funct3: u8, funct7: u8 },
    InvalidRegister { index: u8 },
    InvariantViolation(&'static str),
    DecodeFault(&'static str),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedOpcode { opcode } => {
                write!(f, "unsupported opcode: 0b{opcode:07b}")
            }
            Self::UnsupportedInstruction {
                opcode,
                funct3,
                funct7,
            } => write!(
                f,
                "unsupported instruction: opcode=0b{opcode:07b}, funct3=0b{funct3:03b}, funct7=0b{funct7:07b}"
            ),
            Self::InvalidRegister { index } => write!(f, "invalid register index: x{index}"),
            Self::InvariantViolation(message) => write!(f, "invariant violation: {message}"),
            Self::DecodeFault(message) => write!(f, "decode fault: {message}"),
        }
    }
}

impl std::error::Error for ZkvmError {}
