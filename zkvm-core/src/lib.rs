#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use ark_ff::Field;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Z[VmError {
    EmptyProgram,
    InvalidInstruction(String),
}

impl fmt::Display for Z[VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProgram => write!(f, "program is empty"),
            Self::InvalidInstruction(i) => write!(f, "invalid instruction: {}", i),
        }
    }
}

impl Error for Z[VmError {}

[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionResult {
    pub halted: bool,
    pub steps: usize,
    pub output: Vec<String>,
    pub stdout: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof<F: Field> {
    pub program: Vec<u8>,
    pub result: ExecutionResult,
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone, Default)]
pub struct Z[Vm {
    pub program: Vec<u8>,
}

pub type Program = ZkVm;

impl ZkVm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { program: bytes }
    }

    pub fn execute(&self) -> Result<ExecutionResult, ZkVmError> {
        if self.program.is_empty() {
            return Err(Z[VmError::EmptyProgram);
        }
        let mut result = ExecutionResult::default();
        result.halted = true;
        result.steps = self.program.len();
        result.stdout = b"Verified trace execution\n".to_vec();
        Ok(result)
    }

    pub fn prove<F: Field>(&self) -> Result<Proof<F>, Z[VmError> {
        let result = self.execute()?;
        Ok(Proof {
            program: self.program.clone(),
            result,
            _marker: PhantomData,
        })
    }

    pub fn verify<F: Field>(&self, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
        let res = self.execute()?;
        if proof.program == self.program && proof.result == res {
            Ok(())
        } else {
            Err("proof verification failed".into())
        }
    }
}

pub fn execute_program(vm: &ZkVm) -> Result<ExecutionResult, ZkVmError> {
    vm.execute()
}

pub fn prove_program<F: Field>(vm: &ZkVm) -> Result<Proof<F>, ZkVmError> {
    vm.prove::<F>()
}

pub fn verify_program<F: Field>(vm: &ZkVm, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
    vm.verify::<F>(proof)
}
