#![no_std]

pub mod types;
pub mod vm;

pub use crate::types::{Address, Instruction, RegisterIndex, SignedWord, Word};
pub use crate::vm::{Zkvm, ZkvmConfig, ZkvmError};
