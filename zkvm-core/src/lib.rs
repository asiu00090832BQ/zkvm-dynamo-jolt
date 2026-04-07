use ark_ff::PrimeField;

pub mod elf_loader;
pub mod frontend;
pub mod decoder;
pub mod vm;
pub mod config;
pub mod error;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use elf_loader::{ElfProgram, ElfSegment, SegmentPermissions, ElfLoaderError};
pub use frontend::Frontend;
pub use vm::Zkvm;
pub use config::ZkvmConfig;
pub use error::ZkvmError;
