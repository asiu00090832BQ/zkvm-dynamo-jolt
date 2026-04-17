use crate::decoder::{Instruction, MulDivKind};
use crate::elf_loader::LoadedElf;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZkwmConfig {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zkvm error: {:?}", self)
    }
}

impl Error for ZkvmError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Bumped,
    Ecall,
    Ebreak,
    Halted,
    StepLimitReached,
}

pub struct Zkvm {
    pub regs: [u32; 32],
    pub pc: u32,
    pub memory: Vec<u8>,
    pub config: ZkvmConfig,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0u32; 32],
            pc: 0,
            memory: vec![0u8; config.memory_size],
            config,
        }
    }

    pub fn load_elf_image(&mut self, image: LoadedElf) {
        self.pc = image.entry as u32;
        let len = image.memory.len().min(self.memory.len());
        self.memory[..len].copy_from_slice(&image.memory[..len]);
    }

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> {
        loop {
            let word = self.read_word(self.pc)?;
            let inst = crate::decoder::decode(word).map_err(|_| ZkvmError::DecodeError)?;
            let outcome = self.execute(inst.instruction)?;
            match outcome {
                StepOutcome::Continue => {
                    self.pc = self.pc.wrapping_add(4);
                }
                StepOutcome::Bumped => {}
                _ => return Ok(outcome),
            }
        }
    }

    fn read_word(&self, addr: u32) -> Result<u32, ZkvmError> {
        let addr_usize = addr as usize;
        if addr_usize + 4 > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, len: 4 });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[addr_usize..addr_usize + 4]);
        Ok(u32::from_le_bytes(bytes))
    }

    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, ZkvmError> {
        match inst {
            Instruction::MulDiv { kind, rd, rs1, rs2 } => {
                let lhs = self.regs[rs1 as usize];
                let rhs = self.regs[rs2 as usize];
                let val = match kind {
                    MulDivKind::Mul => mul_u32_wide(lhs, rhs) as u32,
                    MulDivKind::Mulh => (mul_i32_i32_wide(lhs as i32, rhs as i32) >> 32) as u32,
                    MulDivKind::Mulhsu => (mul_i32_u32_wide(lhs as i32, rhs) >> 32) as u32,
                    MulDivKind::Mulhu => (mul_u32_wide(lhs, rhs) >> 32) as u32,
                    MulDivKind::Div => if rhs == 0 { u32::MAX } else { (lhs as i32).wrapping_div(rhs as i32) as u32 },
                    MulDivKind::Divu => if rhs == 0 { u32::MAX } else { lhs.wrapping_div(rhs) },
                    MulDivKind::Rem => if rhs == 0 { lhs } else { (lhs as i32).wrapping_rem(rhs as i32) as u32 },
                    MulDivKind::Remu => if rhs == 0 { lhs } else { lhs.wrapping_rem(rhs) },
                };
                if rd != 0 { self.regs[rd as usize] = val; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Add { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                if rd != 0 { self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]); }
                _Ok(StepOutcome::Continue)
            }
            Instruction::Addi { rd, rs1, imm } => {
                if rd != 0 { self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm as u32); }
                Ok(StepOutcome::Continue)
            }
            Instruction::Lui { rd, imm } => {
                if rd != 0 { self.regs[rd as usize] = imm as u32; }
                Ok(StepOutcome::Continue)
            }
            Instruction::Jal { rd, imm } => {
                let next_pc = self.pc.wrapping_add(imm as u32);
                if rd != 0 { self.regs[rd as usize] = self.pc + 4; }
                self.pc = next_pc;
                Ok(StepOutcome::Bumped)
            }
            Instruction::Ecall => Ok(StepOutcome::Ecall),
            Instruction::Ebreak => Ok(StepOutcome::Ebreak),
            Instruction::Invalid(word) => Err(ZkwmError::InvalidInstruction(word)),
        }
    }
}

fn mul_u32_wide(a: u32, b: u32) -> u64 {
    let a0 = (a & 0xffff) as u64;
    let a1 = (a >> 16) as u64;
    let b0 = (b & 0xffff) as u64;
    let b1 = (b >> 16) as u64;
    a0 * b0 + ((a0 * b1 + a1 * b0) << 16) + ((a1 * b1) << 32)
}

fn mul_i32_i32_wide(a: i32, b: i32) -> i64 {
    (a as i64).wrapping_mul(b as i64)
}

fn mul_i32_u32_wide(a: i32, b: u32) -> i64 {
    (a as i64).wrapping_mul(b as i64)
}
