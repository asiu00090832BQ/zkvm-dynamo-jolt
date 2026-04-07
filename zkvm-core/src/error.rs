use std::fmt;
use crate::decoder::DecodeError;
use crate::elf_loader::ElfLoaderError;

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
        write!(f, "{:?}", self)
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

impl From<ElfLoaderError> for ZkvmError {
    fn from(err: ElfLoaderError) -> Self {
        Self::ElfError(err)
    }
}
