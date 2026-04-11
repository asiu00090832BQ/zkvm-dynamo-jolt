pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, DecodeError, Instruction};
pub use vm::{RunStats, StepOutcome, VmError, Zkvm, ZkvmConfig};
