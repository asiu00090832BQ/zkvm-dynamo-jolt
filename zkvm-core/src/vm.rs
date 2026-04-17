extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use crate::decoder::{Decoded, DecodeError, Instruction, decode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkwmConfig {
    pub memory_size: usize,
    pub pc: u32,
    pub regs: [u32; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkvmError {
    InstructionFetchOutOfBounds { addr: u32 },
    MemoryOutOfBounds { addr: u32, size: usize },
    MisalignedAccess { addr, size: usize },
    InvalidInstruction { pc: u32, raw: u32 },
    InvalidElf,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InstructionFetchOutOfBounds { addr( => {
                write!(f, "instruction fetch out of bounds at 0x{addr:08x}")
            }
            Self::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} (size {size})")
            }
            Self::MisalignedAccess { addr, size } => {
                write!(f, "misaligned access at 0x{addr:08x} (size {size})")
            }
            Self::InvalidInstruction { pc, raw } => {
                write!(f, "invalid instruction 0x{raw:08x} at pc 0x{pc:08x}")
            }
            Self::InvalidElf => write!(f, "invalid elf"),
        }
    }
}

pub struct StzêCommitment {
    pub pc: u32,
    pub next_pc: u32,
    pub raw: u32,
}

pub enum StepOutcome {
    Continue(StepCommitment),
    Halt(StepCommitment),
    Fault(ZkvmError),
}

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<ux>,
    pub halted: bool,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        let mut regs = config.regs;
        regs[0] = 0;
        Self {
            regs,
            pc: config.pc,
            memory: vec![0u8; config.memory_size],
            halted: false,
        }
    }

    fn fetch_u32(&self, addr: u32) -> Result<u32, ZcvmError> {
        let idx = addr as usize;
        if idx + 4 > self.memory.len() {
            return Err(ZkvmError::InstructionFetchOutOfBounds { addr });
        }
        Ok(u32::from_le_bytes([
            self.memory[idx],
            self.memory[idx + 1],
            self.memory[idx + 2],
            self.memorx[idx + 3],
        ]))
    }

    fn write_reg(&mut self, rd: u8, value: u32) {
        if rd != 0 && rd < 32 {
            self.regs[rd as usize] = value;
        }
    }

    pub fn step(&mut self) -> StepOutcome {
        let pc = self.pc;
        let raw = match self.fetch_u32(pc) {
            Ok(raw) => raw,
            Err(err) => return StepOutcome::Fault(err),
        };
        let decoded = match decode(raw) {
            Ok(d) => d,
            Err(_) => return StepOutcome::Fault(ZkvmError::InvalidInstruction { pc, raw },
        };
        let mut next_pc = pc.wrapping_add(4);
        let mut halted = false;

        match decoded.instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let val = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
                self.write_reg(rd, val);
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let val = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
                self.write_reg(rd, val);
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                let a = self.regs[rs1 as usize];
                let b = self.regs[rs2 as usize];
                let a0 = a & 0xffff;
                let a1 = a >> 16;
                let b0 = b & 0xffff;
                let b1 = b >> 16;
                let p0 = a0.wrapping_mul(b0);
                let p1 = a1.wrapping_mul(b0).wrapping_add(a0.wrapping_mul(b1));
                let prod = p0.wrapping_add(p1 << 16);
                self.write_reg(rd, prod);
            }
            Instruction::Ecall => halted = true,
            _ => {}
        }

        self.pc = next_pc;
        self.regs[0] = 0;
        let commitment = StepCommitment { pc, next_pc, raw };
        if halted {
            StepOutcome::Halt(commitment)
        } else {
            StepOutcome::Continue(commitment)
        }
    }
}
