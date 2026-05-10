pub mod vm;
pub mod decoder;
pub mod elf_loader;
pub mod proof;

pub use vm::{Zkvm, ZkvmError, ZkvmConfig};
pub use decoder::*;