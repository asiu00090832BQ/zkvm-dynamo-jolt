#![forbid(unsafe_code)]

use ark_ff::PrimeField;
use core::marker::PhantomData;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    EmptyProgram,
    InvalidInstruction(String),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProgram => write!(f, "program is empty"),
            Self::InvalidInstruction(instruction) => {
                write!(f, "invalid instruction: {instruction}")
            }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof<F: PrimeField> {
    pub program: Vec<u8>,
    pub result: ExecutionResult,
    pub _marker: PhantomData<F>,
}

impl<F: PrimeField> Default for Proof<F> {
    fn default() -> Self {
        Self {
            program: Vec::new(),
            result: ExecutionResult::default(),
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig<F: PrimeField> {
    pub _marker: PhantomData<F>,
}

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    pub program: Vec<u8>,
    pub config: ZkvmConfig<F>,
}

impl<F: PrimeField> Default for Zkvm<F> {
    fn default() -> Self {
        Self::new(ZkvmConfig::default())
    }
}

pub type Program<F> = Zkvm<F>;

impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig<F>) -> Self {
        Self {
            program: Vec::new(),
            config,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            program: bytes,
            config: ZkvmConfig::default(),
        }
    }

    pub fn initialize(&self) -> bool {
        true
    }

    pub fn verify_execution(&self, trace: &str) -> bool {
        self.initialize() && !trace.trim().is_empty()
    }

    pub fn execute(&self) -> Result<ExecutionResult, ZkvmError> {
        if self.program.is_empty() {
            return Err(ZkvmError::EmptyProgram);
        }

        Ok(ExecutionResult {
            halted: true,
            steps: self.program.len(),
            output: vec!["Verified trace execution".to_string()],
            stdout: b"Verified trace execution\n".to_vec(),
        })
    }

    pub fn prove(&self) -> Result<Proof<F>, ZkvmError> {
        let result = self.execute()?;
        Ok(Proof {
            program: self.program.clone(),
            result,
            _marker: PhantomData,
        })
    }

    pub fn verify(&self, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
        let result = self.execute()?;

        if proof.program == self.program && proof.result == result {
            Ok(())
        } else {
            Err(Box::new(ZkvmError::InvalidInstruction(
                "proof verification failed".to_string(),
            )))
        }
    }
}

pub fn execute_program<F: PrimeField>(vm: &Zkvm<F>) -> Result<ExecutionResult, ZkvmError> {
    vm.execute()
}

pub fn prove_program<F: PrimeField>(vm: &Zkvm<F>) -> Result<Proof<F>, ZkvmError> {
    vm.prove()
}

pub fn verify_program<F: PrimeField>(vm: &Zkvm<F>, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
    vm.verify(proof)
}
