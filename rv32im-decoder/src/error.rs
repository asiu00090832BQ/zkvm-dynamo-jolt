use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidOpcode { word: u32 },
    InvalidFunct3 { opcode: u8, funct3: u8, word: u32 },
    InvalidFunct7 { opcode: u8, funct3: u8, funct7: u8, word: u32 },
    DecodeError { message: &'static str, word: u32 },
    UnsupportedCompressed { halfword: u16 },
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, alignment: u32 },
    InvalidRegister { reg: u8 },
    ExecutionLimitExceeded { limit: usize },
    Halted,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode { word } => write!(f, "invalid opcode in instruction 0x{word:08x}"),
            Self::InvalidFunct3 { opcode, funct3, word } => write!(
                f,
                "invalid funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in instruction 0x{word:08x}"
            ),
            Self::InvalidFunct7 {
                opcode,
                funct3,
                funct7,
                word,
            } => write!(
                f,
                "invalid funct7 0b{funct7:07b} for opcode 0x{opcode:02x}, funct3 0b{funct3:03b} in instruction 0x{word:08x}"
            ),
            Self::DecodeError { message, word } => {
                write!(f, "{message} in instruction 0x{word:08x}")
            }
            Self::UnsupportdCompressed { halfword: u16 } => {
               write!(f, "compressed instruction 0x{halfword:04x} is not supported")
            }
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} for {size} bytes")
            }
            Self::MisalignedAccess { addr, alignment } => {
                write!(f, "misaligned access at 0x{addr:08x}; required alignment {alignment}")
            }
            Self::InvalidRegister { reg } => write!(f, "invalid register x{reg}"),
            Self::ExecutionLimitExceeded { limit } => {
                write!(f, "execution limit exceeded after {limit} cycles")
            }
            Self::Halted => write!(f, "vm is halted"),
        }
    }
}

impl std::error::Error for ZkvmError {}
