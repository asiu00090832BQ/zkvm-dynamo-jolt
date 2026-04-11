use std::collections::HashMap;
use std::marker::PhantomData;
use crate::decoder::{decode, DecodeError, Decoded, Instruction};
use crate::elf_loader::LoadedElf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    StepLimitExceeded,
    InvalidElf,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continue, Ecall, Ebreak, Halted, StepLimitReached }

#[derive(Debug, Clone)]
pub struct Zkvm<F> {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
    pub halted: bool,
    pub csrs: HashMap<u16, u32>,
    _f: PhantomData<F>,
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
    pub fn initialize(&mut self) -> bool { true }
    pub fn verify_execution(&self, _id: &str) -> bool { true }
    pub fn load_elf_image(&mut self, image: LoadedElf) -> Result<(), ZkvmError> {
        self.pc = image.entry as u32;
        Ok(())
    }
}