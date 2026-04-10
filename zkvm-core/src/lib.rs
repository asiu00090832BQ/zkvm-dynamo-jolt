pub mod elf_loader;
pub mod vm;

pub use rv32im_decoder::{decode, Instruction, BranchOp, LoadOp, StoreOp, AluOp, AluImmOp, MulDivOp};
pub use crate::elf_loader::{load_elf, ElfLoaderError, LoadedProgram};
pub use crate::vm::{VmExitReason, Zkvm, ZkvmError};

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
