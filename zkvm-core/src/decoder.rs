use core::fmt;

pub use rv32im_decoder::m_extension::{
    decompose_i32_16, decompose_u32_16, mul_i32_u32_wide_16, mul_i32_wide_16, mul_u32_wide_16,
    recompose_i32_16, recompose_u32_16,
};
pub use rv32im_decoder::Instruction;

use rv32im_decoder::{decode as decode_impl, DecodeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
    Halted,
    PcOutOfBounds { pc: u32 },
    MisalignedInstruction { pc: u32 },
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedLoad { addr: u32, width: usize },
    MisalignedStore { addr: u32, width: usize },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(f, "{error}"),
            Self::Halted => write!(f, "zkvm is halted"),
            Self::PcOutOfBounds { pc } => write!(f, "program counter out of bounds: 0x{pc:08x}"),
            Self::MisalignedInstruction { pc } => {
                write!(f, "misaligned instruction fetch at 0x{pc:08x}")
            }
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds: addr=0x{addr:08x}, size={size}")
            }
            Self::MisalignedLoad { addr, width } => {
                write!(f, "misaligned load: addr=0x{addr:08x}, width={width}")
            }
            Self::MisalignedStore { addr, width } => {
                write!(f, "misaligned store: addr=0x{addr:08x}, width={width}")
            }
        }
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
    decode_impl(word).map_err(Into::into)
}
