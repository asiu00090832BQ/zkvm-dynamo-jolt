#![forbid(unsafe_code)]

pub mod vm;
pub mod elf_loader;

pub use rv32im_decoder as decoder;

pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use decoder::{Instruction, Decoded, DecodeSelectors, decode};
pub use elf_loader::{LoadedElf, load_elf};
