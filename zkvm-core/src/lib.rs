#![forbid(unsafe_code)]

pub mod vm;
pub mod decoder;
pub mod elf_loader;
pub mod proof;

pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use decoder::{Instruction, Decoded, HierSelectors, decode};
pub use elf_loader::{LoadedElf, load_elf};
