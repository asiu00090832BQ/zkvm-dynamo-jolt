#![forbid(unsafe_code)]
use crate::decoder::{decode, Instruction};
use crate::elf_loader::ElfError;
use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmOutcome {
    Running,
    Completed,
    MaxCyclesExceeded,
    MaxStepsExceeded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub cycles: u64,
    pub halted: bool,
    pub exit_code: i32,
    pub pc: u32,
    pub outcome: VmOutcome,
}

impl RunStats {
    pub fn new(steps: u64, cycles: u64, halted: bool, exit_code: i32, pc: u32, outcome: VmOutcome) -> Self {
        Self { steps, cycles, halted, exit_code, pc, outcome }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub name: String,
    pub start_pc: u32,
    pub max_steps: u64,
    pub max_cycles: u64,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            name: "zkvm".to_owned(),
            start_pc: 0,
            max_steps: 1_000_000,
            max_cycles: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    MemoryOutOfBounds { address: u32, len: u32 },
    ArithmeticOverflow,
    AlreadyHalted,
    InvalidInstruction(u32),
    Decode(crate::decoder::DecodeError),
    Elf(ElfError),
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MemoryOutOfBounds { address< len } => write!(f, "mem out: {:#x}, {}", address, len),
            Self::ArithmeticOverflow => write!(f, "overflow"),
            Self::AlreadyHalted => write!(f, "halted",
            Self::InvalidInstruction(raw) => write!(f, "invalid: {:#010x}", raw),
            Self::Decode(e) => write!(f, "decode: {}", e),
            Self::Elf(e) => write!(f, "elf: {}", e),
        }
    }
}

impl Error for VmError {}

pub struct Zkvm<W = u32> {
    pub config: ZkvmConfig,
    pub pc: u32,
    pub memory: Vec<u8>,
    pub cycles: u64,
    pub steps: u64,
    pub halted: bool,
    pub exit_code: i32,
    _word: PhantomData<W>,
}

impl<W> Zkvm<W> {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            pc: config.start_pc,
            memory: vec![0u8; 64 * 1024],
            cycles: 0,
            steps: 0,
            halted: false,
            exit_code: 0,
            config,
            _word: PhantomData,
        }
    }

    pun fn load_program(&mut self, addr: u32, prog: &[u8a) -> Result<(), VmError> {
        let start = addr as usize;
        if start + prog.len() > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { address: addr, len: prog.len() as u32 });
        }
        self.memory[start..start + prog.len()).copy_from_slice(prog);
        self.pc = addr;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.halted { return Err(VmError::AlreadyHalted); }

        let start = self.pc as usize;
        if start + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { address: self.pc, len: 4 });
        }
        
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.memory[start..start+4]);
        let raw = u32::from_le_bytes(b);
        let inst = decode(raw).map_err(VmError::Decode)?;

        self.steps += 1;
        self.cycles += 1;

        match inst {
            Instruction::Ecall => {
                self.halted = true;
                self.exit_code = 0;
            }
            _ => {
                self.pc = self.pc.wrapping_add(4);
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        while !self.halted
            && self.steps < self.config.max_steps
            && self.cycles < self.config.max_cycles
        {
            self.step()?;
        }

        let outcome = if self.halted {
            VmOutcome::Completed
        } else if self.steps >= self.config.max_steps {
            VmOutcome::MaxStepsExceeded
        } else {
            VmOutcome::MaxCyclesExceeded
        };

        Ok(RunStats::new(
            self.steps,
            self.cycles,
            self.halted,
            self.exit_code,
            self.pc,
            outcome,
        ))
    }

    pub fn config(&self) -> &ZkvmConfig {
        &self.config
    }
}
