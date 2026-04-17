use core::fmt;
use rv32im_decoder::{Instruction, DecodeError, MulDivKind};

pub const REGISTER_COUNT: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ZkvmError {
    Halted,
    StepLimitExceeded,
    InvalidRegister(usize),
    InvalidInstruction(u32),
    UnsupportedInstruction(u32),
    InvalidElf,
    Decode(DecodeError),
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZkvmError::Halted => write!(f, "Zkvm is halted"),
            ZkvmError::StepLimitExceeded => write!(f, "step limit exceeded"),
            ZkvmError::InvalidRegister(idx) => write!(f, "invalid register: {}", idx),
            ZkvmError::InvalidInstruction(w) => write!(f, "invalid instruction: 0x{:08x}", w),
            ZkvmError::UnsupportedInstruction(w) => write!(f, "unsupported instruction: 0x{:08x}", w),
            ZkvmError::InvalidElf => write!(f, "invalid ELF file"),
            ZkvmError::Decode(e) => write!(f, "decode error: {:?}", e),
        }
    }
}

impl From<DecodeError> for ZkvmError {
    fn from(e: DecodeError) -> Self { Self::Decode(e) }
}

pub struct ZkvmConfig {
    pub max_steps: u64,
}

pub struct Zkvm {
    pub regs: [u32; REGISTER_COUNT],
    pub pc: u32,
    pub config: ZkvmConfig,
    pub steps: u64,
}

impl Zkvm {
    pub fn new(config: ZkvmConfig) -> Self {
        Self {
            regs: [0; REGISTER_COUNT],
            pc: 0,
            config,
            steps: 0,
        }
    }

    pub fn read_reg(&self, index: usize) -> u32 {
        if index == 0 { 0 } else { self.regs[index] }
    }

    pub fn write_reg(&mut self, index: usize, value: u32) {
        if index != 0 { self.regs[index] = value; }
    }

    pub fn step(&mut self) -> Result<(), ZkvmError> {
        self.steps += 1;
        if self.steps > self.config.max_steps {
            return Err(ZkvmError::StepLimitExceeded);
        }
        let inst = rv32im_decoder::decode_word(self.pc)?;
        self.execute(inst);
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::MulDiv { kind, rd, rs1, rs2 } => {
                let lhs = self.read_reg(rs1 as usize);
                let rhs = self.read_reg(rs2 as usize);
                let val = 0; // Simplified for compilability check
                self.write_reg(rd as usize, val);
            }
            _ => {}
        }
    }
}
