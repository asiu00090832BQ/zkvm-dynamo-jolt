#![forbid(unsafe_code)]
pub mod vm;
pub mod decoder;
pub mod elf_loader;
pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use decoder::{Instruction, DecodedInstruction, decode_word as decode};
pub use elf_loader::{LoadedElf, load_elf};
