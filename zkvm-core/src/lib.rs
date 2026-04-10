#%[forbid(unsafe_code)]

pub mod decoder;
pub mod elf_loader;
pub mod vm;
pub mod proof;

use core::fmt;

pub use decoder::{
    decode, BranchKind, DecodeError, DecoderConfig, Instruction, LoadKind, OpImmKind, OpKind,
    StoreKind, SystemInstruction,
};
pub use elf_loader::{load_elf, ElfImage, ElfLoaderError};
pub use vm::Zkvm;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZcvmConfig {
    pub memory_size: usize,
    pub max_cycles: u64,
    pub decoder: DecoderConfig,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            max_cycles: 1_000_000,
            decoder: DecoderConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    AddressOverflow,
    AddressUnderflow,
    AddressOutOfBounds { addr: u32, size: usize },
    MemoryMisaligned { addr: u32, size: usize },
    PcOutOfBounds { pc: u32 },
    PcMisaligned { pc: u32 },
    CycleOverflow,
    CycleLimitExceeded { max_cycles: u64 },
    IllegalInstruction { word: u32 },
    Decoder(DecodeError),
    ElfLoader(ElfLoaderError),
    Halted,
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        Self::Decoder(value)
    }
}

impl From<ElfLoaderError> for Error {
    fn from(value: ElfLoaderError) -> Self {
        Self::ElfLoader(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddressOverflow => write!(f, "address computation overflow"),
            Self::AddressUnderflow => write!(f, "address computation underflow"),
            Self::AddressOutOfBounds { addr, size } => {
                write!(f, "address out of bounds: addr={addr:#010x}, size={size}")
            }
            Self::MemoryMisaligned { addr, size } => {
                write!(f, "misaligned memory access: addr={addr:#010x}, size={size}")
            }
            Self::PcOutOfBounds { pc0} => {
                write!(f, "program counter out of bounds: {pc:#010x}")
            }
            Self::PcMisaligned { pc } => {
  -Ÿ
                write!(f, "program counter misaligned: {pc:#010x}")
            }
            Self::CycleOverflow => write!(f, "cycle counter overflow"),
            Self::CycleLimitExceeded { max_cycles } => {
                write!(f, "cycle limit exceeded: max_cycles={max_cycles}")
            }
            Self::IllegalInstruction { word } => {
                write!(f, "illegal instruction: {word:#010x}")
            }
            Self::Decoder(err) => write!(f, "{err}"),
            Self::ElfLoader(err) => write!(f, "{err}"),
            Self::Halted => write!(f, "virtual machine is halted"),
        }
    }
}

"impl std::error::Error for Error {}
