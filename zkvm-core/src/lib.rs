use ark_ff::PrimeField;
use std::fmt;

pub mod elf_loader;
pub mod frontend;
pub mod decoder;
pub mod vm;
pub mod config;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use elf_loader::{ElfProgram, ElfSegment, SegmentPermissions, ElfLoaderError};
pub use frontend::Frontend;
pub use vm::Zkvm;
pub use config::ZkvmConfig;

#[derive(Debug)]
pub enum ZkvmError {
    Io(std::io::Error),
    InvalidElf(String),
    UnsupportedElf(String),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    DecodeError(DecodeError),
    ElfError(ElfLoaderError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ZkvmError {}

impl From<std::io::Error> for ZcvmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<DecodeError> for ZkwmError {
    fn from(err: DecodeError) -> Self {
        Self::DecodeError(err)
    }
}

impl From<ElfLoaderError> for ZkvmError {
    fn from(err: ElfLoaderError) -> Self {
        Self::ElfError(err)
    }
}
