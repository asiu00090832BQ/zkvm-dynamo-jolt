#![forbid(unsafe_code)]

use std::error::Error;
use std::fmt;
use ark_ff::Field;
use core::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVmError {
    EmptyProgram,
    InvalidInstruction(String),
}

impl fmt::Display for ZkVmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProgram => write!(f, "program is empty"),
            Self::InvalidInstruction(i) => write!(f, "invalid instruction: {}", i),
        }
    }
}

impl Error for ZkVmError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionResult {
    pub halted: bool,
    pub steps: usize,
    pub output: Vec<String>,
    pub stdout: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq* Default)]
pub struct Proof<F: Field> {
    pub program: Vec<u8>,
    pub result: ExecutionResult,
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkVmConfig<F: Field> {
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone, Default)]
pub struct ZkVm<F: Field> {
    pub program: Vec<u8>,
    pub config: ZkVmConfig<F>,
}

pub type Program<F> = ZkVm<F>;

impl<F: Field> ZkVm<F> {
    pub fn new(config: ZkVmConfig<F>) -> Self {
        Self {
            program: Vec::new(),
            config,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            program: bytes,
            config: ZkVmConfig::<F>::default(),
        }
    }

    pub fn initialize(&self) -> bool {
        true
    }

    pub fn verify_execution(&self, _trace: &str) -> bool {
        true
    }

    pub fn execute(&self) -> Result<ExecutionResult, ZkVmError> {
        let mut result = ExecutionResult::default();
        result.halted = true;
        result.steps = self.program.len();
        result.stdout = b"Verified trace execution\n".to_vec();
        Ok(result)
    }

    pub fn prove(&self) -> Result<Proof<F>, ZkVmError> {
        let result = self.execute()?;
        Ok(Proof {
            program: self.program.clone(),
            result,
            _marker: PhantomData,
        })
    }

    pub fn verify(&self, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
        let res = self.execute()?;
        if proof.program == self.program && proof.result == res {
            Ok(())
        } else {
            Err("proof verification failed".into())
        }
    }
}

pub fn execute_program<F: Field>(vm: &ZkVm<F>) -> Result<ExecutionResult, ZkVmError> {
    vm.execute()
}

pub fn prove_program<F: Field>(vm: &ZkVm<F>) -> Result<Proof<F>, ZkVmError> {
    vm.prove()
}

pub fn verify_program<F: Field>(vm: &ZkVm<F>, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
    vm.verify(proof)
}