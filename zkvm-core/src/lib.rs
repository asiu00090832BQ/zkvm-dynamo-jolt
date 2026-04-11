//! zkvm-core: Core components for the Mauryan zkVM on arkworks 0.5.0.
//!
//! This crate aligns workspace exports and provides the `Zkvm` symbol,
//! a field alias for `ark_bn254::Fr`, and integration with the RV32IM
//! decoder and the RISC-V VM engine.

#![forbid(unsafe_code)]

pub mod decoder;
pub mod elf_loader;
pub mod vm;

pub use crate::decoder::{decode32, DecodeError, Decoder, Instruction, Rv32imDecoder};
pub use crate::vm::{
    Csr, ElfInfo, HaltReason, StepCommitment, StepOutcome, Trap, Zkvm, ZkvmConfig, ZkvmError,
};

/// Field alignment for the zkVM: BN254 scalar field.
pub type Field = ark_bn254::Fr;

/// Version string for diagnostics.
pub const VERSION: &str = "zkvm-core/rv32im-arkworks-0.5.0";
