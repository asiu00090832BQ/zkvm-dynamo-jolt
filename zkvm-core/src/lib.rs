pub mod vm;
pub mod proof;
pub mod elf_loader;

pub use vm::{Zkvm, ZkvmError, ZkvmConfig};
pub use rv32im_decoder::{DecodeError, Instruction};
