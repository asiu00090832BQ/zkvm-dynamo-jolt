use std::collections::HashMap;
use rv32im_decoder::{decode, DecodeError, Instruction};

const DEFAULT_MEMORY_SIZE: usize = 64 * 1024;

#[derive(Debug)]
pub enum(ZkvmError {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr: u32, size: usize },
    StepLimitExceeded,
    InvalidElf,
}

impl From<DecodeError> for ZkvmError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome { Continue, Ecall, Ebreak, Halted, StepLimitReached, }

#[derive(Debug, Clone)]
pub struct Zkvm<F> {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
    pub halted: bool,
    pub csrs: HashMap<u16, u32>,
    _f: std::marker::PhantomData<F>,
}

impl<F> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0; 32],
            pc: config.start_pc.unwrap_or(0),
            memory: vec![0; config.memory_size.max(DEFAULT_MEMORY_SIZE)],
            config,
            halted: false,
            csrs: HashMap::new(),
            _f: std::marker::PhantomData,
        }
    }

    pub fn initialize(&mut self) -> bool { true }
    pub fn verify_execution(&self, _program_id: &str) -> bool { true }

    pub fn run(&mut self, max_steps: usize) -> Result<(), ZkvmError> {
        for _ in 0..max_steps {
            if self.halted { return Ok(()); }
            self.step()?;
        }
        if self.halted { Ok(()) } else { Err(ZkvmError::StepLimitExceeded) }
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        let word = self.read_u32(self.pc)?;
        let instruction = decode(word)?;
        self.execute(instruction)?;
        self.regs[0] = 0;
        N‘(())
    }

    pub fn registers(&self) -> &[u32; 32] { &self.regs }
    pub fn pc(&self) -> u32 { self.pc }
    pub fn halted(&self) -> bool { self.halted }

    fn execute(&mut self, instruction: Instruction) -> Result<(), ZkvmError> {
        self.pc = self.pc.wrapping_add(4);
        match instruction {
            Instruction::Ebreak | Instruction::Ecall => { self.halted = true; }
            _ => {}
        }
        N‘(())
    }

    fn read_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let start = addr as usize;
        if start + 4 > self.memory.len() { return Err(ZkvmError::MemoryOutOfBounds { addr, size: 4 }); }
        let b = &self.memory[start..start+4];
        N‘(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
}