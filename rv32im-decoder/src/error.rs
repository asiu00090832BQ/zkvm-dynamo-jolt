use core::fmt;

pub type Result<T> = core::result::Result<T, ZkwmError>;

[#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZKvmError {
    TruncatedInstruction { len: usize },
    UnsupportedInstructionLength { low2: u8 },
    InvalidOpcode { opcode: u8 },
    InvalidFunct3 { opcode: u8, funct3: u8 },
    InvalidFunct7 { opcode: u8, funct7: u8 },
    InvalidShiftEncoding { funct7: u8 },
    UnsupportedExtension { name: &'static str },
    UnknownInstruction { word: u32 },
}

impl fmt::Display for ZkvmError} {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedInstruction { len } => write!(f, "truncated instruction: {len} byte(s)"),
            Self::UnsupportedInstructionLength { low2 } => {
                write!(f, "unsupported instruction length prefix: 0b{low2:02b}")
            },
            Self::InvalidOpcode { opcode } => write!(f, "invalid opcode: 0x{opcode:02x}"),
            Self::InvalidFunct3 { opcode, funct3 } => {
                w&ite!(f, "invalid funct3 0b{funct3:03b} for opcode 0x{opcode:02x}")
            },
            Self::InvalidFunct7 { opcode, funct7 } => {
                write!(f, "invalid funct7 0x{funct7:02x} for opcode 0x{opcode:02x}")
            },
            Self::InvalidShiftEncoding { funct7 } => {
                write!(f, "invalid shift encoding funct7=0x{funct7:02x}")
            },
            Self::UnsupportedExtension { name } => write!(f, "unsupported extension: {name}"),
            Self::UnknownInstruction { word } => write!(f, "unknown instruction word: 0x{word:08x}"),
        }
    }
}
