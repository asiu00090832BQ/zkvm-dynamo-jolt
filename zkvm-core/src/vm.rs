use std::fmt;

use crate::decoder::{decode, Instruction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub mem_size: usize,
    pub max_steps: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trap {
    IllegalInstruction,
    Breakpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HaltReason {
    Ecall,
    StepLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Halted(HaltReason),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError {
    MemoryOverflow,
    ElfLoadBounds,
    StepLimit,
    DecodeError,
    InvalidElf,
    Trap(Trap),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::MemoryOverflow => write!(f, "memory access overflow"),
            ZkvmError::ElfLoadBounds => write!(f, "ELF image does not fit in VM memory"),
            ZkvmError::StepLimit => write!(f, "step limit exceeded"),
            ZkvmError::DecodeError => write!(f, "instruction decode error"),
            ZkvmError::InvalidElf => write!(f, "invalid ELF image"),
            ZkvmError::Trap(Trap::IllegalInstruction) => write!(f, "illegal instruction trap"),
            ZkvmError::Trap(Trap::Breakpoint) => write!(f, "breakpoint trap"),
        }
    }
}

impl std::error::Error for ZkvmError {}

#[derive(Debug, Clone)]
pub struct Zkvm {
    config: ZkvmConfig,
    memory: Vec<u8>,
    regs: [u32; 32],
    pc: u32,
    steps: u64,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            config,
            memory: vec![0; config.mem_size],
            regs: [0; 32],
            pc: 0,
            steps: 0,
        }
    }

    pub fn load_image(&mut self, image: &[u8]) -> Result<(), ZkvmError> {
        if image.is_empty() {
            return Err(ZkvmError::InvalidElf);
        }

        if image.len() > self.memory.len() {
            return Err(ZkvmError::ElfLoadBounds);
        }

        self.memory.fill(0);
        self.memory[..image.len()].copy_from_slice(image);
        self.regs = [0; 32];
        self.pc = 0;
        self.steps = 0;
        Ok(())
    }

    pub fn run(&mut self) -> Result<HaltReason, ZkvmError> {
        loop {
            match self.step()? {
                StepOutcome::Continue => {}
                StepOutcome::Halted(reason) => return Ok(reason),
            }
        }
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.steps >= self.config.max_steps {
            return Ok(StepOutcome::Halted(HaltReason::StepLimit));
        }

        let word = self.load_u32(self.pc)?;
        let decoded = decode(word)?;

        match decoded.instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let value = self.regs[rs1].wrapping_add(self.regs[rs2]);
                self.write_reg(rd, value);
                self.pc = self.pc.wrapping_add(4);
                self.steps += 1;
                Ok(StepOutcome::Continue)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let value = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                self.write_reg(rd, value);
                self.pc = self.pc.wrapping_add(4);
                self.steps += 1;
                Ok(StepOutcome::Continue)
            }
            Instruction::Ecall => {
                self.pc = self.pc.wrapping_add(4);
                self.steps += 1;
                Ok(StepOutcome::Halted(HaltReason::Ecall))
            }
            Instruction::Ebreak => Err(ZkvmError::Trap(Trap::Breakpoint)),
            Instruction::Invalid(_) => Err(ZkvmError::Trap(Trap::IllegalInstruction)),
        }
    }

    pub fn load_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let addr = addr as usize;
        let end = addr.checked_add(4).ok_or(ZkvmError::MemoryOverflow)?;
        if end > self.memory.len() {
            return Err(ZkvmError::MemoryOverflow);
        }

        let bytes = [
            self.memory[addr],
            self.memory[addr + 1],
            self.memory[addr + 2],
            self.memory[addr + 3],
        ];
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn write_reg(&mut self, idx: usize, value: u32) {
        if idx != 0 && idx < self.regs.len() {
            self.regs[idx] = value;
        }
    }
}
