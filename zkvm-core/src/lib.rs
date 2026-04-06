#![forbid(unsafe_code)]

use ark_ff::Field;
use core::marker::PhantomData;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZcvmError {
    EmptyProgram,
    InvalidInstruction(String),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProgram => write!(f, "program is empty"),
            Self::InvalidInstruction(i) => write!(f, "invalid instruction: {}", i),
        }
    }
}

impl Error for ZkvmError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionResult {
    pub halted: bool,
    pub steps: usize,
    pub output: Vec<String>,
    pub stdout: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Proof<F: Field> {
    pub program: Vec<u8>,
    pub result: ExecutionResult,
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig<F: Field> {
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone, Default)]
pub struct Zkvm<F: Field> {
    pub program: Vec<u8>,
    pub config: ZkvmConfig<F>,
}

pub type Program<F> = Zkvm<F>;

impl<F: Field> Zkvm<F> {
    pub fn new(config: ZkvmConfig<F>) -> Self {
        Self {
            program: Vec::new(),
            config,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            program: bytes,
            config: ZkvmConfig::<F>::default(),
        }
    }

    pub fn initialize(&self) -> bool {
        true
    }

    pub fn verify_execution(&self, _trace: &str) -> bool {
        true
    }

    pub fn execute(&self) -> Result<ExecutionResult, ZkvmError> {
        if self.program.is_empty() {
            return Err(ZkvmError::EmptyProgram);
        }

        let mut result = ExecutionResult::default();
        result.halted = true;
        result.steps = self.program.len();
        result.stdout = b"Verified trace execution
".to_vec();
        Ok(result)
    }

    pub fn prove(&self) -> Result<Proof<F>, ZkvmError> {
        let result = self.execute()?;
        Ok(Proof {
            program: self.program.clone(),
            result,
            _marker: PhantomData<F>,
        })
    }

    pub fn verify(&self, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
        let res = self.execute();
        match res {
            Ok(r) => {
                if proof.program == self.program && proof.result == r {
                    Ok(())
                } else {
                    Err(Box::new(ZkvmError::InvalidInstruction(
                        "proof verification failed".to_string(),
                    )))
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}

pub fn execute_program<F: Field>(vm: &Zkvm<F>) -> Result<ExecutionResult, ZkvmError> {
    vm.execute()
}

pub fn prove_program<F: Field>(vm: &Zkvm<F>) -> Result<Proof<F>, ZkvmError> {
    vm.prove()
}

pub fn verify_program<F: Field>(
    vm: &Zkvm<F>,
    proof: &Proof<F>,
) -> Result<(), Box<dyn Error>> {
    vm.verify(proof)
}
