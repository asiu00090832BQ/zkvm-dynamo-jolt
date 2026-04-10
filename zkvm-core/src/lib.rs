#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Core components for the Zkvm project.
//!
//! The crate is intentionally structured into three primary runtime modules:
//! - [`decoder`]: Instruction decoding.
//! - [`elf_loader`]: Hardened ELF parsing and segment extraction.
//! - [`vm`]: A small VM that can execute a subset of RV64 instructions.
//!
//! A proof-system integration module may also exist elsewhere in the repository.

/// Instruction decoding support.
pub mod decoder;
/// Hardened ELF parsing and segment extraction.
pub mod elf_loader;
/// Proof-system integration.
pub mod proof;
/// Virtual-machine execution support.
pub mod vm;

/// Canonical decoder exports.
pub use decoder::{DecodeError, DecoderConfig, Instruction};
/// Backward-compatible decoder alias.
pub use decoder::DecodedInstruction;
/// Canonical ELF loader exports.
pub use elf_loader::{ElfImage, ElfLoaderError};
/// Backward-compatible ELF loader exports.
pub use elf_loader::{ElfError, ElfLoader, ElfSegment};
/// Canonical VM exports.
pub use vm::{Error, Zkvm, ZkvmConfig};
/// Backward-compatible VM error alias.
pub use vm::VmError;
