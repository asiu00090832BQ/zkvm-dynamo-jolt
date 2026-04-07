pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use crate::decoder::{decode, AluOp, BranchKind, Instruction, LoadWidth, MulOp, StoreWidth};
pub use crate::elf_loader::{load_elf, LoadSegment, LoadedElf};
pub use crate::error::{Result as ZkvmResult, ZkvmConfig, ZkvmError};
pub use crate::vm::Zkvm;
