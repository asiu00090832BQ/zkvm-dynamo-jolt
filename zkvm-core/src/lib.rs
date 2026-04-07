pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use decoder::{
    decode, AluOp, BranchKind, DecodeError, Instruction, LoadWidth, MulOp, StoreWidth,
};
pub use elf_loader::{
    load_elf, ElfLoadError, LoadSegment, LoadedElf, SegmentFlags,
};
pub use error::{ZkvmConfig, ZkvmError};
pub use vm::Zkvm;
