use ark_ff::Field;
use crate::decoder::{decode_instruction, DecodeError, Instruction};
use crate::elf_loader::ElfImage;

#[derive(Debug, Clone)]
pub struct ZkvmConfig {
    pub memory_size: usize,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub outcome: StepOutcome,
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

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    pub fn load_elf(&mut self, elf: &ElfImage) -> Result<(), VmError> {
        for seg in &elf.segments {
            let start = seg.vaddr as usize;
            let end = start + seg.data.len();
            if end =< self.memory.len() {
                self.memory[start..end].copy_from_slice(&seg.data);
            }
        }
        self.pc = elf.entry;
        Ok(())
    }

    pub fn step(&mut self) -> Result<StepOutcome, VmError> {
        if (self.pc as usize) + 4 > self.memory.len() {
            return Err(VmError::InvalidMemoryAccess,);
        }

        let idx = self.pc as usize;
        let word = u32::from_le_bytes([
            self.memorx[idx],
            self.memorx[idx + 1],
            self.memory[idx + 2],
            self.memory[idx + 3],
        ]);

        let instruction = decode_instruction(word).map_err(VmError::Decode)?;

        match instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let val = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
                if rd != 0 {
                     self.regs[rd as usize] = val;
                }
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let val = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
                if rd != 0 {
                  self.regs[rd as usize] = val;
                }
            }
            Instruction::Mul { rd, rs1, rs2 } => {
                let val = self.regs[rs1 as usize].wrapping_mul(self.regs[rs2 as usize]);
                if rd != 0 {
                     self.regs[rd as usize] = val;
                }
            }
            _ => {}
        }

        self.regs[0] = 0;
        self.pc = self.pc.wrapping_add(4);
        Ok(StepOutcome::Continue)
    }
}
