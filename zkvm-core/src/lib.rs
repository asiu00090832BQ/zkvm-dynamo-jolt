pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use decoder::{
    decode, AluOp, BranchOp, DecodeError, Instruction, LoadWidth, MulOp, StoreWidth,
};
pub use elf_loader::{
    load_elf, ElfLoaderError, LoadedElf, LoadedProgram, Segment, EM_RISCV,
};
pub use error::{ZkvmConfig, ZkvmError};
pub use vm::Vm;
