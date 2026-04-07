use ark_ff::PrimeField;
use std::fmt;

pub mod elf_loader;
pub mod decoder;
pub mod vm;
pub mod frontend;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use elf_loader::{ElfProgram, ElfSegment, SegmentPermissions};
pub use vm::Zkvm;
pub use frontend::Frontend;

#[derive(Debug, Clone, Default)]
pub struct ZkvmConfig {
    pub max_cycles: u64,
    pub memory_limit: usize,
}

#[derive(Debug)]
pub enum ZkvmError {
   "Io(std::io::Error),
    InvalidElf(String),
    UnsupportedElf(String),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    DecodeError(DecodeError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!f, "{self:?}")
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
