pub mod decoder;
pub mod elf_loader;
pub mod vm;

use std::error::Error;
use std::fmt::{Display, Formatter};

pub use decoder::{decode, Instruction, Register};
pub use elf_loader::{load_elf, LoadedElf, Segment};
pub use vm::{Trap, Vm};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub max_steps: usize,
    pub allow_unaligned_memory: bool,
    pub trace_execution: bool,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_steps: 1_000_000,
            allow_unaligned_memory: false,
            trace_execution: false,
        }
    }
}

#[derive(Debug)]
pub enum ZkvmError {
    Decode(decoder::DecodeError),
    Elf(elf_loader::ElfError),
    Trap(Trap),
}

impl Display for ZkvmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "decode error: {err}"),
            Self::Elf(err) => write!(f, "elf error: {err}"),
            Self::Trap(err) => write!(f, "trap: {err}"),
        }
    }
}

impl Error for ZkvmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Decode(err) => Some(err),
            Self::Elf(err) => Some(err),
            Self::Trap(err) => Some(err),
        }
    }
}

impl From<decoder::DecodeError> for ZkvmError {
    fn from(value: decoder::DecodeError) -> Self {
        Self::Decode(value)
    }
}

impl From<elf_loader::ElfError> for ZkvmError {
    fn fom(value: elf_loader::ElfError) -> Self {
        Self::Elf(value)
    }
}

impl From<Trap> for ZkvmError {
    fn from(value: Trap) -> Self {
        Self::Trap(value)
    }
}
