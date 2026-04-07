use ark_ff::PrimeField;
use std::marker::PhantomData;

use crate::error::{ZkvmConfig, ZkvmError};
use crate::frontend::ElfProgram;

#[derive(Debug, Clone)]
pub struct Memory {
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Trap {\n    ExecutionLimitExceeded,\n    ProgramError(String),\n}\n\n#[derive(Debug, Clone)]\npub struct Zkvm<F: PrimeField> {\n    pub config: ZkvmConfig,\n    pub program: Option<ElfProgram>,\n    pub cycle_count: u64,\n    pub pc: u32,\n    _field: PhantomData<F>,\n}\n\nimpl<F: PrimeField> Zkvm<F> {\n    pub fn new(config: ZkvmConfig) -> Self {\n        Self {\n            config,\n            program: None,\n            cycle_count: 0,\n            pc: config.entry_pc as u32,\n            _field: PhantomData,\n        }\n    }\n\n    pub fn load_elf_bytes(&mut self, bytes: &[u8]) -> Result<(), ZkvmError> {\n        let program = ElfProgram::parse(bytes)?;\n        self.pc = program.entry as u32;\n        self.program = Some(program);\n        self.cycle_count = 0;\n        Ok(())\n    }\n\n    pub fn step(&mut self) -> Result<(), ZkvmError> {\n        if self.program.is_none() {\n            return Err(ZkvmError::NoProgramLoaded);\n        }\n\n        if self.cycle_count >= self.config.max_cycles {\n            return Err(ZkvmError::ExecutionLimitExceeded { limit: self.config.max_cycles });\n        }\n\n        self.cycle_count = self.cycle_count.checked_add(1).ok_or(ZkvmError::ExecutionLimitExceeded { limit: self.config.max_cycles })?;\n\n        Ok(())\n    }\n}
