use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZkvmError {
    InvalidOpcode { opcode: u8, word: u32 },
    InvalidFunct3 { opcode: u8, funct3: u8, word: u32 },
    InvalidFunct7 { opcode: u8, funct7: u8, word: u32 },
    InvalidRegister { index: u8 },
    InvalidShiftImmediate { shamt: u8, word: u32 },
    UnsupportedInstruction { word: u32 },
    MisalignedInstruction { pc: u32 },
    MemoryOutOfBounds { address: u32, size: usize },
    MisalignedLoad { address: u32, size: usize },
    MisalignedStore { address: u32, size: usize },
    Ecall,
    Ebreak,
}

pub type ZkvmResult<T> = core::result::Result<T, ZkvmError>;

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode { opcode, word } => {
                write!(f, "invalid opcode 0x{opcode:02x} in word 0x{word:08x}")
            }
            Self::InvalidFunct3 {
                opcode,
                funct3,
                word,
            } => write!(
                f,
                "invalid funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in word 0x{word:08x}"
            ),
            Self::InvalidFunct7 {
                opcode,
                funct7,
                word,
            } => write!(
                f,
                "invalid funct7 0b{funct7:07b} for opcode 0x{opcode:02x} in word 0x{word:08x}"
            ),
            Self::InvalidRegister { index } => write!(f, "invalid register x{index}"),
            Self::InvalidShiftImmediate { shamt, word } => {
                write!(f, "invalid shift immediate {shamt} in word 0x{word:08x}")
            }
            Self::UnsupportedInstruction { word } => {
                write!(f, "unsupported instruction 0x{word:08x}")
            }
            Self::MisalignedInstruction { pc } => {
                write!(f, "misaligned instruction fetch at 0x{pc:08x}")
            }
            Self::MemoryOutOfBounds { address, size } => {
                write!(f, "memory access out of bounds at 0x{address:08x} for {size} bytes")
            }
            Self::MisalignedLoad { address, size } => {
                write!(f, "misaligned load at 0x{address:08x} for {size} bytes")
            }
            Self::MisalignedStore { address, size } => {
                write!(f, "misaligned store at 0x{address:08x} for {size} bytes")
            }
            Self::Ecall => write!(f, "ecall trap"),
            Self::Ebreak => write!(f, "ebreak trap"),
        }
    }
}
