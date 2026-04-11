pub mod decoder;
pub mod elf_loader;
pub mod vm;
pub use decoder::{decode, Instruction, DecodeError};
pub use vm::{Zkvm, ZkvmConfig, VmError, RunStats, StepOutcome};
