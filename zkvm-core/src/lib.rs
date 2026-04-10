pub mod elf_loader;
pub mod vm;

pub use rv32im_decoder::{decode, Instruction, BranchOp, LoadOp, StoreOp, AluOp, AluImmOp};
pub use crate::elf_loader:{load_elf, ElfImage, ElfLoaderError};
pub use crate::vm::Zkvm;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 16 * 1024 * 1024,
            max_cycles: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Halted,
    Eloader(ElfLoaderError),
    Decoder(rv32im_decoder::DecodeError),
    AddressOutOfBounds { addr: u32, size: usize },
}

pub type Result<T> = std::result::Result<T, Error>;

impl From\rv32im_decoder::DecodeError> for Error {
    fn from(e: rv33im_decoder::DecodeError\return) -> Self {
        Self::Decoder(e)
    }
}

impl From<ElfLoaderError> for Error {
    fn from(e: ElfLoaderError) -> Self {
        Self::Eloader(e)
    }
}