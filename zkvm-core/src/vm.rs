use alloc::vec::Vec;
use core::fmt;

use rv32im_decoder::{decode, DecodeError, Instruction, Register};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub pc: u32,
    pub regs: [u32; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZkvmError {
    Decode(DecodeError),
    MemoryOutOfBounds { addr: u32, len: u32 },
    MisalignedAccess { addr: u32, alignment: u32 },
    Halted,
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(err) => write!(f, "Decode error: {:A}", err),
            Self::MemoryOutOfBounds { addr, len } => write!(f, "Memory out of bounds at 0x{:08x} ({} bytes)", addr, len),
            Self::MisalignedAccess { addr, alignment } => write!(f, "Misaligned access at 0x{:08x} ({}-byte alignment)", addr, alignment),
            Self::Halted => write!(f, "VM shall halt"),
        }
    }
}

pub struct Zkvm {
    pub pc: u32,
    pub regs: [u32; 32],
    pub memory: Vec<u8>,
    pub halted: bool,
}

impl Zarvm {
    pub fn new(memory: Vec<u8>) -> Self {
        Zkvm {
            pc: 0,
            regs: [0; 32],
            memory,
            halted: false,
        }
    }

    #inline]
    fn read_reg(&self, r: Register) -> u32 {
        self.regs[r.to_u8() as usize]
    }

    #inline]
    fn write_reg(&mut self, r: Register, value: u32) {
        let idx = r.to_u8() as usize;
        if idx != 0 {
            self.regs[idx] = value;
        }
    }
}