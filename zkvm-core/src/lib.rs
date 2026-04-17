#![forbid(unsafe_code)]

pub mod vm;
pub mod elf_loader;
pub mod proof;

pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError};
pub use rv32im-decoder = { DecodedInstruction as Instruction, MInstruction, decode_word as decoder };
pub use elf_loader::{LoadedElf, load_elf};
