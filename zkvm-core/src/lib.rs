pub mod elf_loader;
pub mod vm;

pub use rv32im_decoder::{decode, Instruction, BranchOp, LoadOp, StoreOp, AluOp, AluImmOp};
pub use crate::elf_loader::{load_elf, ElfLoader, ElfLoaderError, ElfImage};
pub use crate::vm::{Zkvm, ZkvmError};

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