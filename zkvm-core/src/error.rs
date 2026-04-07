use std::fmt;
use crate::decoder::DecodeError;
use crate::elf_loader::ElfLoaderError;

#[derive(Debug)]
pub enum ZcvmError {
    Io(std::io::Error),
    InvalidElf(String),
    UnsupportedElf(String),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    DecodeError(DecodeError),
    ElfError(ElfLoaderError),
}

impl fmt::Display for ZcvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ZcvmError {}

impl From<std::io::Error> for ZcvmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<DecodeError> for ZcvmError {
    fn from(err: DecodeError) -> Self {
        Self::DecodeError(err)
    }
}

impl From<ElfLoaderError> for ZcvmError {
    fn from(err: ElfLoaderError) -> Self {
        Self::ElfError(err)
    }
}
