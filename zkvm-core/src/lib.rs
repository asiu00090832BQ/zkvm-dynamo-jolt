use ark_ff::PrimeField;

pub mod decoder;
pub mod elf_loader;
pub mod error;
pub mod frontend;
pub mod vm;

pub use decoder::{decode, DecodeError, DecoderConfig, Instruction};
pub use elf_loader::{load_elf, ElfLoadError, LoadedElf, LoadSegment, SegmentFlags};
pub use error::{ZkvmConfig, ZkvmError};
pub use vm::{Memory, Trap, Zkvm};
pub use frontend::ElfProgram;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ProgramProof;

pub fn execute_program<F: PrimeField>(_program: &ElfProgram) -> Result<ExecutionResult, ZkvmError> {
    Ok(ExecutionResult { stdout: Vec::new() })
}

pub fn prove_program<F: PrimeField>(_program: &ElfProgram) -> Result<ProgramProof, ZkvmError> {
    Ok(ProgramProof)
}

pub fn verify_program<F: PrimeField>(_program: &ElfProgram, _proof: &ProgramProof) -> Result<(), ZkvmError> {
    Ok(())
}