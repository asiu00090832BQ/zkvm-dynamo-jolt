//! Core library for the zkVM.

#![deny(missing_debug_implementations, missing_docs)]
#![forbid(unsafe_code)]

pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use crate::error::{ZkvmConfig, ZkvmError};
pub use crate::vm::{Memory, Trap, Zkvm};
pub use crate::decoder::{decode, DecodeError, DecodedInstruction};
pub use crate::elf_loader::{load_elf, ElfLoadError, LoadedElf, LoadSegment};
pub use crate::frontend::ElfProgram;

use ark_ff::PrimeField;
use core::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionResult {
    pub cycles: u64,
    pub halted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramProof<F: PrimeField> {
    _field: PhantomData<F>,
}

impl<F: PrimeField> Default for ProgramProof<F> {
    fn default() -> Self {
        Self {
            _field: PhantomData,
        }
    }
}

pub fn execute<F: PrimeField>(
    vm: &mut Zkvm<F>,
    max_cycles: u64,
) -> Result<ExecutionResult, ZkvmError> {
    let mut cycles: u64 = 0;
    while cycles < max_cycles {
        match vm.step() {
            Ok(()) => {
                cycles = cycles.checked_add(1).ok_or(ZkvmError::ExecutionLimitExceeded { limit: max_cycles })?;
            }
            Err(err) => return Err(err),
        }
    }
    Ok(ExecutionResult { cycles, halted: false })
}

pub fn prove<F: PrimeField>(_vm: &Zkvm<F>, _execution: &ExecutionResult) -> Result<ProgramProof<F>, ZkvmError> {
    Ok(ProgramProof::default())
}

pub fn verify<F: PrimeField>(_proof: &ProgramProof<F>) -> Result<(), ZkvmError> {
    Ok(())
}