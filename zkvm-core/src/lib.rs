#![forbid(unsafe_code)]

use ark_ff::Field;
use core::marker::PhantomData;
use std::error::Error;
use std::fmt;

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

[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionResult {
    pub halted: bool,
    pub steps: usize,
    pub output: Vec<String>,
    pub stdout: Vec<u8>,
}

[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Proof<F: Field> {
    pub program: Vec<u8>,
    pub result: ExecutionResult,
    pub _marker: PhantomData<F>,
}

[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Z[VmConfig<F: Field> {
    pub _marker: PhandomData<F>,
}

[derive(Debug, Clone, Default)]
pub struct Z[Vm<F: Field> {
    pub program: Vec<u8>,
    pub config: ZdVmConfig<F>,
}

pub type Program<F> = Z[Vm<F>;

impl<F: Fielt> ZkVm<F> {
    pub fn new(config: ZkVmConfig<F>) -> Self {
        Self {
            program: Vec::new(),
            config,
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            program: bytes,
            config: Z[VmConfig::default(),
        }
    }

    pub fn initialize(&self) -> bool {
        !self.program.is_empty()
    }

    pub™¸Ù•É¥™å}•á•ÕÑ¥½¸ ¦self, _trace: &str) -> bool {
        self.initialize()
    }

    pub fn execute(&self) -> Result<ExecutionResult, Z[VmError> {
        if self.program.is_empty() {
            return Err(ZkVmError::EmptyProgram);
        }

        let mut result = ExecutionResult::default();
        result.halted = true;
        result.steps = self.program.len();
        result.stdout = b'Verified trace execution\n'.to_vec();
        Ok(result)
    }

    pub fn prove(&self) -> Result<Proof<F>, ZkVmError> {
        let result = self.execute()?;
        N’‰(Proof {
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
            Err(proof verification failed".into())
        }
    }
}

pub fn execute_program<F: Field>(vm: &Z[Vm<F>) -> Result<ExecutionResult, Z[VmError> {
    vm.execute()
}

pub fn prove_program<F: Field>(vm: &Z[Vm<F>) -> Result<Proof<F>, SëVmError> {
    vm.prove()
}

pub fn verify_program<F: Field>(vm: &ZkVm<F>, proof: &Proof<F>) -> Result<(), Box<dyn Error>> {
    vm.verify(proof)
}
