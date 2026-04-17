use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidInstruction(u32),
    UnsupportedOpcode(u8),
    UnsupportedFunct3 { opcode: u8, funct3: u8 },
    UnsupportedFunct7 { opcode: u8, funct7: u8 },
    ExtensionDisabled(&'static str),
    RegisterOutOfBounds(usize),
    VerificationFailed(&'static str),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction(word) => write!(f, "invalid instruction word: 0x{word:08x}"),
            Self::UnsupportedOpcode(opcode) => write!(f, "unsupported opcode: 0b{opcode:07b}"),
            Self::UnsupportedFunct3 { opcode, funct3 } => write!(
                f,
                "unsupported funct3 0b{funct3:03b} for opcode 0b{opcode:07b}"
            ),
            Self::UnsupportedFunct7 { opcode, funct7 } => write!(
                f,
                "unsupported funct7 0b{funct7:07b} for opcode 0b{opcode:07b}"
            ),
            Self::ExtensionDisabled(name) => write!(f, "{name} extension is disabled"),
            Self::RegisterOutOfBounds(index) => write!(f, "register index out of bounds: {index}"),
            Self::VerificationFailed(message) => write!(f, "verification failed: {message}"),
        }
    }
}

impl std::error::Error for ZkvmError {}
