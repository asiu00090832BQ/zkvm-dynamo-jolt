pub mod vm;
pub mod decoder;
pub mod elf_loader;

pub use crate::vm::{Zkvm, ZkvmConfig, VmError, RunStats, VmOutcome};
pub use crate::decoder::{decode, Instruction};
