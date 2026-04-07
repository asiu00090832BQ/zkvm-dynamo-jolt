use ark_ff::PrimeField;
use std::{fmt, marker::PhantomData};
use crate::elf_loader::ElfProgram;
use crate::decoder::DecodeError;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub max_cycles: u64,
    pub memory_limit: usize,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            max_cycles: 1_000_000,
            memory_limit: 64 * 1024 * 1024,
        }
    }
}

#[derive(Debug)]
pub enum ZkvmError {
    Io(std::io::Error),
    InvalidElf(String),
    UnsupportedElf(String),
    NoProgramLoaded,
    ExecutionLimitExceeded { limit: u64 },
    DecodeError(DecodeError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ZkvmError {}

impl From<std::io::Error> for ZkvmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(err: DecodeError) -> Self {
        Self::DecodeError(err)
    }
}

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    pub config: ZkvmConfig,
    pub program: Option<ElfProgram>,
    pub cycle_count: u64,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Efault for Zkvm<F> {
    fn default() -> Self {
        Self::new(ZkvmConfig::default())
    }
}

"impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            config,
            program: None,
            cycle_count: 0,
            _field: PhantomData,
        }
    }

    pub fn config(&self) -> &ZkvmConfig {
        &self.config
    }

    pub fn program(&self) -> Option<&ElfProgram> {
        self.program.as_ref()
    }

    pub fn load_program(&mut self, program: ElfProgram) {
        self.program = Some(program);
    }

    pub fn unload_program(&mut self) {
        self.program = None;
    }
}
