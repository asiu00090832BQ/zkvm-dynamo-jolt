pub mod vm;
pub mod decoder;
pub mod elf_loader;
pub mod proof;

pub use vm::{Zkvm, ZkvmError, ZkvmConfig, StepOutcome};
pub use decoder::*;
pub use elf_loader::load_elf;