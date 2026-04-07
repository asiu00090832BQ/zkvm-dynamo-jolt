use std::fmt;

use crate::decoder::DecodeError;

#[derive(Debug)]
pub enum Error {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize, len: usize },
    MisalignedAccess { addr: u32, alignment: usize },
    PcOutOfBounds { pc: u32, len: usize },
    InvalidRegister(usize),
    UnsupportedInstruction(String),
    Halted,
    StepLimitExceeded(usize),
}

pub type Result<T> = std::result::Result<T, Error>;
pub type VmResult<T> = Result<T>;
pub type VMResult<T> = Result<T>;
pub type VmError = Error;
pub type VMError = Error;
pub type ExecError = Error;

impl Error {
    pub fn unsupported<M: Into<String>>(message: M) -> Self {
        Self::UnsupportedInstruction(message.into())
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "decode error: {err:?}"),
            Self::MemoryOutOfBounds { addr, size, len } => {
                write!(
                    f,
                    "memory access out of bounds at 0x{addr:08x} (size {size}, memory len {len})"
                )
            }
            Self::MisalignedAccess { addr, alignment } => {
                write!(f, "misaligned access at 0x{addr:08x} (alignment {alignment})")
            }
            Self::PcOutOfBounds { pc, len } => {
                write!(
                    f,
                    "program counter out of bounds: 0x{pc:08x} (memory len {len})"
                )
            }
            Self::InvalidRegister(index) => write!(f, "invalid register index {index}"),
            Self::UnsupportedInstruction(message) => {
                write!(f, "unsupported instruction: {message}")
            }
            Self::Halted => write!(f, "virtual machine is halted"),
            Self::StepLimitExceeded(limit) => write!(f, "step limit exceeded after {limit} steps"),
        }
    }
}

impl std::error::Error for Error {}
