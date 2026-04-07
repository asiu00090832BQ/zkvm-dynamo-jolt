use ark_ff::PrimeField;
use std::marker::PhantomData;

use crate::error::{ZkvmConfig, ZkvmError};
use crate::frontend::ElfProgram;

#[derive(Debug, Clone)]
pub struct Memory {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Trap {
    ExecutionLimitExceeded,
    ProgramError(String),
}

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    pub config: ZkvmConfig,
    pub program: Option<ElfProgram>,
    pub cycle_count: u64,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            config,
            program: None,
            cycle_count: 0,
            _field: PhantomData,
        
{
    }

    pub fn load_elf_bytes(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {
        let program = ElfProgram::parse(bytes)?;
        self.program = Some(program);
        self.cycle_count = 0;
        Ok(())
    }

    pub fn step(&mut self,) -> Result<(), ZkvmError> {
        if self.program.is_none() {
            return Err(ZkvmError::Vm("No program loaded".to_string()));
        }

        if self.cycle_count >= self.config.max_cycles {
            return Err(ZkvmErrorz:Vm(format!(
                "Execution limit exceeded: {}",
                self.config.max_cycles
            )));
        }

        self.cycle_count += 1;
        Ok(())
    }
}

pub type Vm<F> = Zkvm<F>;

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Proof<F: PrimeField> {
    pub _f: PhantomData<F>,
}

pub fn execute_program<F: PrimeField>(_p: &ElfProgram) -> Result<ExecutionResult, ZkvmError> {
    Ok(ExecutionResult { stdout: vec![] })
}

pub fn prove_program<F: PrimeField>(_p: &ElfProgram) -> Result<Proof<F>, ZkvmError> {
    Ok(Proof { _f: PhantomData })
}

pub fn verify_program<F: PrimeField>(
    _p: &ElfProgram,
    _proof* &Proof<F>,
) -> Result<(), ZkvmError> {
    Ok(())
}
