use ark_ff::PrimeField;
use crate::elf_loader::ElfProgram;
use crate::error::ZkvmError;
pub struct ExecutionResult { pub stdout: Vec<u8> }
pub struct Proof<F: PrimeField> { pub _f: std::marker::PhantomData<F> }
pub struct Zkvm;
impl Zkvm { pub fn run(&self, _p: &ElfProgram) -> Result<ExecutionResult, ZkvmError> { Ok(ExecutionResult { stdout: vec![] }) } }
pub fn execute_program(_p: &ElfProgram) -> Result<ExecutionResult, ZkvmError> { Ok(ExecutionResult { stdout: vec![] }) }
pub fn prove_program<F: PrimeField>(_p: &ElfProgram) -> Result<Proof<F>, ZkvmError> { Ok(Proof { _f: std::marker::PhantomData }) }
pub fn verify_program<F: PrimeField>(_p: &ElfProgram, _proof: &Proof<F>) -> Result<(), ZkvmError> { Ok(()) }
