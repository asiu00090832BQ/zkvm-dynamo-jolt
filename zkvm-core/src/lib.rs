
pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, Instruction};
pub use elf_loader::{load_elf, LoadResult};
pub use vm::{ExecutionResult, HaltReason, StepResult, Zkvm};

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub entry_pc: u32,
    pub max_steps: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            entry_pc: 0,
            max_steps: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInstruction(u32),
    ElfFormat(&static str),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, alignment: usize },
    PcOutOfBounds(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<_>) -> fmt::Result {
        match self {
            Error::InvalidInstruction(word) => {
                write!(f, "invalid or unsupported instruction: 0x{word:08x}")
            }
            Error::ElfFormat(msg) => write!(f, "invalid ELF: {msg}"),
            Error::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} ({size} bytes)")
            }
            Error::MisalignedAccess { addr, alignment } => {
                write!(f, "misaligned access at 0x{addr:08x} (alignment {alignment})")
            }
            Error::PcOutOfBounds(pc) => write!(f, "program counter out of bounds: 0x{pc:08x}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
