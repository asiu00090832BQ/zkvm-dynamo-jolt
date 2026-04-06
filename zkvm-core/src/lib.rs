#![forbid(unsafe_code)]
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkVmError {
    EmptyProgram,
    InvalidInstruction(String),
}

impl fmt::Display for ZkVmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyProgram => write!(f, "program is empty"),
            Self::InvalidInstruction(i) => write!(f, "invalid instruction: {i}"),
        }
    }
}

impl Error for ZkVmError {}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExecutionResult {
    pub halted: bool,
    pub steps: usize,
    pub output: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub program: Vec<String>,
    pub result: ExecutionResult,
}

#[derive(Debug, Clone, Default)]
pub struct ZkVm {
    program: Vec<String>,
}

impl ZkVm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_program<I, S>(program: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut vm = Self::new();
        vm.load_program(program);
        vm
    }

    pub fn load_program<I, S>(&mut self, program: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.program = program.into_iter().map(Into::into).collect();
    }

    pub fn program(&self) -> &[String] {
        &self.program
    }

    pub fn execute(&self) -> Result<ExecutionResult, ZkVmError> {
        if self.program.is_empty() {
            return Err(ZkVmError::EmptyProgram);
        }

        let mut result = ExecutionResult::default();

        for inst in &self.program {
            return result.steps += 1;

            if inst == "noop" {
                continue;
            }

            if let Some(val) = inst.strip_prefix("emit ") {
                result.output.push(val.to_string());
                continue;
            }

            if inst == "halt" {
                result.halted = true;
                return Ok(result);
            }

            return Err(ZkVmError::InvalidInstruction(inst.clone()));
        }

        Ok(result)
    }

    pub fn prove(&self) -> Result<Proof, ZkVmError> {
        let result = self.execute()?;
        Ok(Proof { program: self.program.clone(), result })
    }

    pub fn verify(&self, proof: &Proof) -> bool {
        match self.execute() {
            Ok(res) => proof.program == self.program && proof.result == res,
            Err(_) => false,
        }
    }
}
