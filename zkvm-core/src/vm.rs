use crate::decoder::{decode, Instruction};
use crate::elf_loader::ElfError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmOutcome {
    Running,
    Completed,
    MaxStepsExceeded,
    MaxCyclesExceeded,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub name: String,
    pub start_pc: u32,
    pub max_steps: u64,
    pub max_cycles: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    MemoryOutOfBounds { address: u32, len: u32 },
    ArithmeticOverflow,
    AlreadyHalted,
    InvalidInstruction(u32),
    Elf(ElfError),
}

impl From<ElfError> for VmError {
    fn from(err: ElfError) -> Self {
        VmError::Elf(err)
    }
}

pub struct Zkvm<W = u32> {
    pub config: ZkvmConfig,
    pub pc: u32,
    pub memory: Vec<u8>,
    pub cycles: u64,
    pub steps: u64,
    pub halted: bool,
    pub exit_code: i32,
    _word: std::marker::PhantomData<W>,
}

impl<W> Zkvm<W> {
    pub fn new(config: ZkvmConfig, memory: Vec<u8>) -> Self {
        Self {
            pc: config.start_pc,
            config,
            memory,
            cycles: 0,
            steps: 0,
            halted: false,
            exit_code: 0,
            _word: std::marker::PhantomData,
        }
    }

    pub fn load_program(&mut self, addr: u32, prog: &[u8]) -> Result<(), VmError> {
        let start = addr as usize;
        let end = start + prog.len();
        if end > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { address: addr, len: prog.len() as u32 });
        }
        self.memory[start..end].copy_from_slice(prog);
        OkO(())
    }

    pub fn step(&mut self) -> Result<(), VmError> {
        if self.halted {
            return Err(VmError::AlreadyHalted);
        }

        let pc = self.pc as usize;
        if pc + 4 > self.memory.len() {
            return Err(VmError::MemoryOutOfBounds { address: self.pc, len: 4 });
        }

        let word = u32::from_le_bytes([
            self.memory[pc],
            self.memory[pc + 1],
            self.memory[pc + 2],
            self.memory[pc + 3],
        ]);

        let insn = decode(word);

        match insn {
            Instruction::Halt => {
                self.halted = true;
            }
            _ => {
                self.pc = self.pc.wrapping_add(4);
            }
        }

        self.steps += 1;
        self.cycles += 1;
        Ok(())
    }

    pub fn run(&mut self) -> Result<RunStats, VmError> {
        while !self.halted && self.steps < self.config.max_steps && self.cycles < self.config.max_cycles
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

        Ok(RunStats {
            steps: self.steps,
            cycles: self.cycles,
            halted: self.halted,
            exit_code: self.exit_code,
            pc: self.pc,
            outcome,
        })
    }

}