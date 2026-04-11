//! zkvm-core: Core components for the Mauryan zkVM on arkworks 0.5.0.
//!
//! This crate aligns workspace exports and provides the 'Zkvm' symbol,
//! a field alias for 'ark_bn254::Fr', and integration with the RV32IM
//! decoder and the RISC-V VM engine.

#![forbid(unsafe_code)]

use ark_bn254::Fr;
use core::fmt;

pub mod vm;
pub mod decoder;
pub mod elf_loader;

pub use vm::{Zkvm, ZkvmConfig, StepOutcome, ZkvmError, HaltReason, Trap};
pub use decoder::{Instruction, Decoded, HierSelectors, decode};
pub use elf_loader::{LoadedElf, load_elf};

pub type Field = Fr;