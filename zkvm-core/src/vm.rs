use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedElf;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig { 
    pub memory_size: usize, 
    pub max_cycles: Option<u64>, 
    pub start_pc: Option<u32>, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError { 
    DecodeError, 
    InvalidElf, 
    MemoryOutOfBounds { addr: u32, len: usize }, 
    InvalidInstruction(u32), 
    StepLimitReached, 
    Trap, 
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "zkVM Error: {:?}", self)
    }
}

impl Error for ZkvmError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continue, Ecall, Ebreak, Halted, StepLimitReached, }

pub struct Zkvm { 
    pub regs: [u32; 32], 
    pub pc: u32, 
    pub memory: Vec<u8>, 
    pub config: ZkvmConfig, 
}

impl Zkvm { 
    pub fn new(config: ZkvmConfig) -> Self { 
        Self { regs: [0u32; 32], pc: 0, memory: vec![0u8; config.memory_size], config, } 
    }

    pub fn load_elf_image(&mut self, image: LoadedElf) {
        self.pc = image.entry as u32;
        let len = image.memory.len().min(self.memory.len());
        self.memory[..len].copy_from_slice(&image.memory[..len]);
    }

    pub fn initialize(&mut self) -> bool { true }

    pub fn verify_execution(&self, _input: &str) -> bool { true }

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> { Ok(StepOutcome::Halted) }
}
