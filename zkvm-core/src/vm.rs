use ark_ff::PrimeField;
use std::marker::PhantomData;
use crate::{ZkvmConfig, ZkvmError, ElfProgram};

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
        }
    }

    pub fn load_elf_bytes(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {
        let program =
            ElfProgram::parse(bytes).map_err(|e| ZkvmError::InvalidElf(e.to_string()))?;
        self.program = Some(program);
        self.cycle_count = 0;
        Ok(())
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
}
