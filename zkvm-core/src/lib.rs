use ark_ff::PrimeField;
use std::{fmt, marker::PhantomData};

pub mod frontend;

pub use frontend::{ElfProgram, ElfSegment, Frontend};

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::InvalidElf(message) => write!(f, "invalid ELF: {message}"),
            Self::UnsupportedElf(message) => write!(f, "unsupported ELF: {message}"),
            Self::NoProgramLoaded => write!(f, "no program loaded"),
            Self::ExecutionLimitExceeded { limit } => {
                write!(f, "execution limit exceeded after {limit} cycles")
            }
        }
    }
}

impl std::error::Error for ZkvmError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ZkvmError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[derive(Debug, Clone)]
pub struct Zkvm<F: PrimeField> {
    pub config: ZkvmConfig,
    program: Option<ElfProgram>,
    cycle_count: u64,
    _field: PhantomData<F>,
}

impl<F: PrimeField> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            config,
            program: None,
            cycle_count: 0,
            _field: PhantomData,
        }
    }

    pub fn load_elf_bytes(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {
        let program = Frontend::load_elf(bytes)?;
        self.program = Some(program);
        self.cycle_count = 0;
        Ok(())
    }

    pub fn load_elf_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), ZkvmError> {
        let program = Frontend::load_elf_file(path)?;
        self.program = Some(program);
        self.cycle_count = 0;
        Ok(())
    }

    pub fn program(&self) -> Option<&ElfProgram> {
        self.program.as_ref()
    }

    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        if self.program.is_none() {
            return Err(ZkvmError::NoProgramLoaded);
        }

        if self.cycle_count >= self.config.max_cycles {
            return Err(ZkvmError::ExecutionLimitExceeded {
                limit: self.config.max_cycles,
            });
        }

        self.cycle_count += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ZkvmError> {
        let steps = self
            .program
            .as_ref()
            .ok_or(ZkvmError::NoProgramLoaded)?
            .entry_point_steps_hint()
            .max(1);

        for _ in 0..steps {
            self.step()?;
        }

        Ok(())
    }
}
