use ark_ff::Field;

use crate::decoder::{decode_instruction, DecodeError, Instruction};

pub struct ZkvmConfig {
    pub memory_size: usize,
}

pub struct ElfImage {
    pub entry: u64,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum VmError {
    Decode(DecodeError),
    InvalidMemoryAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Halted,
}

pub struct Zkvm<F: Field> {
    pub config: ZkvmConfig,
    pub pc: u32,
    pub regs: [u32; 32],
    pub memory: Vec<u8>,
    _marker: core::marker::PhantomData<F>,
}

impl<F: Field> Zkvm<F> {
    pub fn new(config: ZkvmConfig) -> Self {
        let memory = vec![0u8; config.memory_size];
        Zkvm {
            config,
            pc: 0,
            regs: [0; 32],
            memory,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn load_elf(&mut self, elf: &ElfImage) -> Result<(), VmError> {
        let len = core::cmp::min(elf.data.len(), self.memory.len());
        self.memory[..len].copy_from_slice(&elf.data[..len]);
        self.pc = elf.entry as u32;
        Ok(())
    }

    pub fn step(&mut self) -> Result<StepOutcome, VmError> {
        if (self.pc as usize) + 4 > self.memory.len() {
            return Err(VmError::InvalidMemoryAccess);
        }

        let idx = self.pc as usize;
        let word = u32::from_le_bytes([
            self.memory[idx],
            self.memory[idx + 1],
            self.memory[idx + 2],
            self.memory[idx + 3],
        ]);

        let instruction = decode_instruction(word).map_err(VmError::Decode)?;

        match instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let value = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let value = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                let value = self.regs[rs1 as usize].wrapping_mul(self.regs[rs2 as usize]);
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let a = self.regs[rs1 as usize] as i64;
                let b = self.regs[rs2 as usize] as i64;
                let value = ((a * b) >> 32) as u32;
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let a = self.regs[rs1 as usize] as u64;
                let b = self.regs[rs2 as usize] as u64;
                let value = ((a * b) >> 32) as u32;
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
            Instruction::Remu { rd, rs1, rs2 } => {
                let divisor = self.regs[rs2 as usize];
                let value = if divisor == 0 {
                    self.regs[rs1 as usize]
                } else {
                    self.regs[rs1 as usize] % divisor
                };
                if rd != 0 {
                    self.regs[rd as usize] = value;
                }
            }
        }

        self.regs[0] = 0;

        self.pc = self.pc.wrapping_add(4);
        Ok(StepOutcome::Continue)
    }
}
