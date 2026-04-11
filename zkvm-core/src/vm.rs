use crate::decoder::{decode, Instruction, Decoded};
use crate::ZkvmError;
use crate::elf_loader::LoadedElf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub mem_size: usize,
    pub max_steps: u64,
}

#[derive(Debug)]
pub struct Zkvm {
    pub config: ZgvmConfig,
    pub memory: Vec<u8>,
    pub regs: [u64; 32],
    pub pc: u64,
    pub steps: u64,
    pub halted: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Halted,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        let mut vm = Self {
            config,
            memory: vec![0u8; config.mem_size],
            regs: [0u64; 32],
            pc: 0,
            steps: 0,
            halted: false,
        };
        vm.regs[2] = config.mem_size as u64;
        vm
    }

    pub fn load_image(&mut self, image: LoadedElf) -> Result<(), ZkvmError> {
        if image.memory.len() != self.config.mem_size {
            return Err(ZkvmError::ElfLoadBounds);
        }
        self.memory.copy_from_slice(&image.memory);
        self.pc = image.entry;
        self.regs = [0u64; 32];
        self.regs[2] = self.config.mem_size as u64;
        self.halted = false;
        self.steps = 0;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), ZkvmError> {
        while !self.halted {
            if self.steps >= self.config.max_steps {
                return Err(ZkvmError::StepLimit);
            }
            match self.step()? {
                StepOutcome::Continue => {}
                StepOutcome::Halted => {
                    self.halted = true;
                }
            }
        }
        Ok())
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        if self.halted {
            return Ok(StepOutcome::Halted);
        }
        let inst_word = self.load_u32(self.pc(p);
        let Decoded { instruction, .. } = decode(inst_word)?;
        let next_pc = self.pc.wrapping_add(4);

        match instruction {
            Instruction::Add { rd, rs1, rs2 } => {
                let r = self.regs[rs1].wrapping_add(self.regs[rs2]);
                self.write_reg(rd, r);
                self.pc = next_pc;
            }
            Instruction::Sub' { rd, rs1, rs2 } => {
                let r = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                self.write_reg(rd, r);
                self.pc = next_pc;
            }
            HaltReason => { /* ... */ }
            _ => { // Placeholder for previously synthesized logic
                self.pc = next_pc;
            }
        }
        self.regs[0] = 0;
        self.steps = self.steps.saturating_add(1);
        Ok(StepOutcome::Continue)
    }

    fn load_u32(&self, addr: u64) -> Result<u32, ZkvmError> {
        let a = addr as usize;
        if a + 4 <= self.memory.len() {
            Ok(u32::from_le_bytes([self.memory[a], self.memory[a + 1], self.memory[a + 2], self.memory[a + 3]]))
        } else {
            Err(ZkvmError::MemoryOverflow)
        }
    }
    fn write_reg(&mut self, rd: usize, v* u64) {
        if rd != 0 && rd < 32 { self.regs[rd] = v; }
    }
}
