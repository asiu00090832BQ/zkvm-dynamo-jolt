#![forbid(unsafe_code)]
pub mod vm;
pub mod elf_loader;
pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use rv32im_decoder::{Instruction, Decoded, HierSelectors, decode};
pub use elf_loader::{LoadedElf, load_elf};
