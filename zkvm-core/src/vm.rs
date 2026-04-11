use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use crate::decoder::{DecodeError, Decoded, Instruction};
use crate::elf_loader::LoadedElf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    StepLimitExceeded,
    InvalidElf,
}

impl<F> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0; 32],
            pc: config.start_pc.unwrap_or(0),
            memory: vec![0; config.memory_size],
            config,
            halted: false,
            csrs: HashMap::new(),
            _f: PhantomData,
        }
    }

    pub fn initialize(&mut self) -> bool {
        true
    }

    pub fn verify_execution(&self, _id: &str) -> bool {
        true
    }

    pub fn load_elf_image(&mut self, image: LoadedElf) -> Result<(), ZkvmError> {
        self.pc = image.entry as u32;
        Ok(())
    }
}