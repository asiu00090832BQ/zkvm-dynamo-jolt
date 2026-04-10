#![forbid(unsafe_code)]

pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use decoder::{decode, DecodeError, Instruction};
pub use elf_loader::{parse_elf, ElfError, ElfImage, LoadableSegment};
pub use vm::{RunStats, StepOutcome, VmError, Zkvm, ZkvmConfig};
