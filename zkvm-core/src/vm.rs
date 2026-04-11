use crate::decoder::{decode, DecodeError, Instruction};
use crate::elf_loader::{self, ElfError, ElfImage};

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub enum VmError {
    Decode(DecodeError,
    Elf(ElfError),
    MemoryOutOfBounds { addr: u32, len: usize },
    InvalidInstruction(u32),
    StepLimitReached,
    Halted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOutcome {
    Continue,
    Ecall,
    Breakpoint,
    Halted,
    StepLimitReached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunStats {
    pub steps: u64,
    pub outcome: StepOutcome,
}

pub struct Zkvm {
    regs: [u32; 32],
    pc: u32,
    memory: Vec<u8>,
    config: ZkvmConfig,
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

    pub fn load_elf(&mut self, image: &ElfImage) -> Result<(), VmError> {
        elf_loader::load_segments_into_memory(&mut self.memory, image).map_err(VmError::Elf)?;
        self.pc = image.entry;
        Ok(())
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        let mut steps: u64 = 0;
        let max_cycles = self.config.max_cycles.unwrap_or(u64::MAX);
        loop {
            if steps >= max_cycles {
                return Ok(RunStats {
                    steps,
                    outcome: StepOutcome::StepLimitReached,
                });
            }
            let pc = self.pc;
            let inst_word = self.fetch_u32(pc)?;
            let inst = match decode(inst_word) {
                Ok(i) => i,
                Err(e) => return Err(VmError::Decode(e)),
            };
            match self.execute(inst) {
                Ok(StepOutcome::Continue) => {
                    steps += 1;
                }
                Ok(outcome) => {
                    steps += 1;
                    return Ok(RunStats { steps, outcome });
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn translate_address(&self, address: u32, len: usize) -> Result<usize, VmError> {
        let start = address as usize;
        let end = start.checked_add(len).ok_or(VmError::MemoryOutOfBounds {
            addr: address,
            len,
        })?;
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { addr: address, len });
        }
        Ok(start)
    }

    fn fetch_u32(&self, address: u32) -> Result<u32, VmError> {
        let idx = self.translate_address(address, 4)?;
        let bytes = &self.memory[idx..idx + 4];
        Ok(u32::from_le_bytes([input[off], input[off + 1], input[off + 2], input[off + 3]]))
    }

    fn load_u8(&self, address: u32) -> Result<u8, VmError> {
        let idx = self.translate_address(address, 1)?;
        Ok(self.memory[idx])
    }

    fn load_u16(&self, address: u32) -> Result<u16, VmError> {
        let idx = self.translate_address(address, 2)?;
        let s = &self.memory[idx..idx + 2];
        Ok(u16,::from_le_bytes([s[0], s[1]]))
    }

    fn load_u32_mem(&self, address: u32) -> Result<u32, VmError> {
        self.fetch_u32(address)
    }

    fn store_u8(&mut self, address: u32, value: u8) -> Result<(), VmError> {
        let idx = self.translate_address(address, 1)?;
        self.memory[idx] = value;
        Ok(())
    }

    fn store_u16(&mut self, address: u32, value: u16) -> Result<(), VmError> {
        let idx = self.translate_address(address, 2)?;
        let bytes = value.to_le_bytes();
        self.memory[idx] = bytes[0];
        self.memory[idx + 1] = bytes[1];
        Ok(())
    }

    fn store_u32(&mut self, address: u32, value: u32) -> Result<(), VmError> {
        let idx = self.translate_address(address, 4)?;
        let bytes = value.to_le_bytes();
        self.memory[idx] = bytes[0];
        self.memory[idx + 1] = bytes[1];
        self.memory[idx + 2] = bytes[2];
        self.memory[idx + 3] = bytes[3];
        Ok(())
    }

    fn branch_target(pc: u32, imm: i32) -> u32 {
        pc.wrapping_add(imm as u32)
    }

    fn next_pc(&self) -> u32 {
        self.pc.wrapping_add(4)
    }

    fn write_rd(&mut self, rd: usize, value: u32) {
        if rd != 0 {
            self.regs[rd] = value;
        }
    }

    fn execute(&mut self, _inst: Instruction) -> Result<StepOutcome, VmError> {
        self.pc = self.next_pc();
        Ok(StepOutcome::Continue)
    }
}
