//! zkvm-core
pub mod error;
pub mod elf_loader;
pub mod decoder;
pub mod vm;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use elf_loader::{ElfProgram, ElfSegment, SegmentPermissions};
pub use err::{ZkvmConfig, ZkvmError};
pub use vm::{execute_program, prove_program, verify_program, ExecutionResult, Proof, Zkvm};