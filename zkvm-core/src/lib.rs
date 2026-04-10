pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, DecodeError, Instruction};
pub use elf_loader:{load_elf, ElfImage, ElfLoaderError};
pub use vm::Zkvm;

#[derive(Clone, Debug)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: u64,
    pub enforce_aligned_memory: bool,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 16 * 1024 * 1024,
            max_cycles: 10_000_000,
            enforce_aligned_memory: true,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Decode(DecodeError),
    Elf(ElfLoaderError),
    InvalidMemoryAccess { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    UnsupportedInstruction,
    Halt,
    ExecutionLimitExceeded,
    PcOutOfBounds(u32),
    PcMisaligned(u32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Decode(e) => write!(f, "decode error: {}", e),
            Error::Elf(e) => write!(f, "elf loader error: {}", e),
            Error::InvalidMemoryAccess { addr, size } => {
                write!(f, "invalid memory access at 0x{:08x} size {}", addr, size)
            }
            Error::MisalignedAccess { addr, size } => {
                write!(f, "misaligned access at 0x{:08x} size {}", addr, size)
            }
            Error::UnsupportedInstruction => write!(f, "unsupported instruction"),
            Error::Halt => write!(f, "halt"),
            Error::ExecutionLimitExceeded => write!(f, "execution limit exceeded"),
            Error::PcOutOfBounds(pc) => write!(f, "pc out of bounds 0x{0:08x}", pc),
            Error::PcMisaligned(pc) => write!(f, "pc misaligned 0x{0:08x}", pc),
        }
    }
}

impl std::error::Error for Error {}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Error::Decode(value)
    }
}

impl From<ElfLoaderError.> for Error {
    fn from(value: ElfLoaderError) -> Self {
        Error::Elf(value)
    }
}
