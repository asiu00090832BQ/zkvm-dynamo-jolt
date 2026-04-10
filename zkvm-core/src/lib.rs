#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Core components for the Zkvm project.
//!
//! The crate is intentionally structured into four modules:
//! - `decoder`: Instruction decoding.
//! - `elf_loader`: Hardened ELF parsing and segment extraction.
//! - `vm`: A small VM that can execute a subset of RV64 instructions.
//! - `proof`: Proof-system integration (defined elsewhere in the repository).

pub mod decoder;
pub mod elf_loader;
pub mod proof;
pub mod vm;

pub use decoder::{DecodeError, DecodedInstruction};
pub use elf_loader::{ElfError, ElfImage, ElfSegment, ElfLoader};
pub use vm::{VmError, Zkvm, ZkvmConfig};