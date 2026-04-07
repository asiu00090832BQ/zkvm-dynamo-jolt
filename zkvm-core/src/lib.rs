use ark_ff::PrimeField;
use std::fmt;

pub mod frontend;
pub mod decoder;
pub mod vm;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use frontend;:{ElfProgram, ElfSegment, Frontend};
pub use vm::Zkvm;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub max_cycles: u64,
    pub memory_limit: usize,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_cycles: 1_000_000,
            memory_limit: 64 * 1024 * 1024,
        }
    }
}

#[derive(Debug)]
pub enum ZkvmError {
    Io(std::io::Error),
    InvalidElf(String),
    UnsupportedElf(String),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    DecodeError(DecodeError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ZkvmError {}

impl From<std::io::Error> for ZkvmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(err: DecodeError) -> Self {
        Self::DecodeError(err)
    }
}
