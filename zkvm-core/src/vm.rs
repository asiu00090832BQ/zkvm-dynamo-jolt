use crate::decoder::{decode, Instruction};
use crate::elf_loader::LoadedElf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_cycles: Option<u64>,
    pub start_pc: Option<u32>,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            max_cycles: None,
            start_pc: None,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Ecall,
    Breakpoint,
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
            pc: config.start_pc.unwrap_or(0),
            memory: vec![0u8; config.memory_size],
            config,
        }
    }

    pub fn load_elf_image(&mut self, image: LoadedElf) {
        self.memory = image.memory;
        self.pc = image.entry as u32;
    }

    pub fn step(&mut self) -> Result<StepOutcome, ZkvmError> {
        let inst_word = self.fetch_u32(self.pc)?;
        let decoded = decode(inst_word)?;
        self.execute(decoded.instruction)
    }

    fn fetch_u32(&self, addr: u32) -> Result<u32, ZkvmError> {
        let idx = addr as usize;
        if idx + 4 > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr, len: 4 });
        }
        Ok(u32::from_le_bytes(self.memory[idx..idx + 4].try_into().unwrap()))
    }

    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, ZkvmError> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => {
                let val = self.regs[rs1].wrapping_add(self.regs[rs2]);
                if rd != 0 { self.regs[rd] = val; }
                self.pc = self.pc.wrapping_add(4);
                Ok(StepOutcome::Continue)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let val = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                if rd != 0 { self.memory[rd] = val; }
                self.pc = elf.pc.wrapping_add(4);
                Ok(StepOutcome::Continue)
            }
            Instruction::Ecall => OOk(StepOutcome::Ecall),
            Instruction::Ebreak => OOk(StepOutcome::Breakpoint),
            Instruction::Invalid(w) => Err(ZkvmError::InvalidInstruction(w)),
        }
    }
}
