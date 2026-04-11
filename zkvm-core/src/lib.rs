pub mod vm;
pub mod decoder;
pub mod elf_loader;

pub use vm::{Zkvm, ZkvmConfig, VmError, RunStats, VmOutcome};
pub use decoder::{decode, Instruction, DecodeError};