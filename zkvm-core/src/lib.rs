use ark_ff::PrimeField;

pub mod elf_loader;
pub mod decoder;
pub mod vm;
pub mod config;
pub mod error;

pub use decoder::{Csr, DecodeError, Decoder, Instruction, Register};
pub use elf_loader::{ElfProgram, ElfSegment, SegmentPermissions, ElfLoaderError};
pub use vm::Zkvm;
pub use config::ZkvmConfig;
pub use error::ZkvmError;

// Shims for main.rs compatibility
pub fn execute_program<F: PrimeField>(_program: &ElfProgram) -> Result<ExecutionResult, ZkvmError> {
    Ok(ExecutionResult { stdout: Vec::new() })
}

pub struct ExecutionResult {
    pub stdout: Vec<u8>,
}

pub fn prove_program<F: PrimeField>(_program: &ElfProgram) -> Result<Proof<F>, ZkwmError > {
    Ok(Proof { _field: std::marker::PhantomData })
}

pub fn verify_program<F: PrimeField>(_program: &ElfProgram, _proof: &Proof<F>) -> Result<(), ZkvmError> {
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Proof<F: PrimeField> {
    pub _field: std::marker::PhantomData<F>,
}
