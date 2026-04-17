#![forbid(unsafe_code)]

pub mod vm;
pub mod elf_loader;
pub mod proof;

pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use rv32im_decoder::{DecodedInstruction as Instruction, MInstruction};
pub use elf_loader::{LoadedElf, load_elf};
