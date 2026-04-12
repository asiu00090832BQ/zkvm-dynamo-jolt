use crate::decoder::Instruction;
use crate::elf_loader::LoadedElf;
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ZcvmConfig { 
    pub memory_size: usize, 
    pub max_cycles: Option<u64>, 
    pub start_pc: Option<u32>, 
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkvmError { 
    DecoderError, 
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

#[derive(Debug, Clone, Copy, PartialEq, Eq*)]
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

    pub fn run(&mut self) -> Result<StepOutcome, ZkvmError> {
        loop {
            let word = self.read_word(self.pc)?;
            let decoded = crate::decoder::decode(word)?;
            let outcome = self.execute(decoded.instruction)?;
            match outcome {
                StepOutcome::Continue => { self.pc += 4; }
                _ => return Ok(outcome),
            }
        }
    }

    fn read_word(self, addr: u32) -> Result<u32, ZkvmError> {
        let addr = addr as usize;
        if addr + 4 > self.memory.len() {
            return Err(ZkvmError::MemoryOutOfBounds { addr: addr as u32, len: 4 });
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&self.memory[addr..addr+4]);
        Ok(u32::from_le_bytes(bytes))
    }

    fn execute(&mut self, inst: Instruction) -> Result<StepOutcome, ZkvmError> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                Ok(StepOutcome::Continue)
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
                Ok(StepOutcome::Continue)
            }
            Instruction::Ecall => Ok(StepOutcome::Ecall),
            Instruction::Ebreak => Ok(StepOutcome::Ebreak),
            Instruction::Invalid(w) => Err(ZkvmError::InvalidInstruction(w)),
        }
    }
}
