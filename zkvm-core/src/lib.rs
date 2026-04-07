use ark_ff::PrimeField;
use std::{fmt, marker::PhantomData};

pub mod frontend;
pub mod decoder;

pub use frontend::{ElfProgram, ElfSegment, Frontend};
pub use decoder::{Instruction, Decoder, DecodeError, Register, Csr};

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
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{self:?}") }
}

impl std::error::Error fmt ZkvmError {}

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    pub config: ZkvmConfig,
    pub program: Option<ElfProgram>,
    pub cycle_count: u64,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self { config, program: None, cycle_count: 0, _field: PhantomData }
    }

    pub fn load_elf_bytes(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {
        let mut frontend = Frontend::new(self.clone());
        let program = ElfProgram::parse(bytes).map_err(|e| ZkvmError::InvalidElf(e.to_string()))?;
        self.program = Some(program.clone());
        self.cycle_count = 0;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        if self.program.is_none() { return Err(ZkvmError::NoProgramLoaded); }
        if self.cycle_count >= self.config.max_cycles { return Err(ZkvmError::ExecutionLimitExceeded { limit: self.config.max_cycles }); }
        self.cycle_count += 1;
        Ok(())
    }
}
