pub mod decoder;
pub mod elf_loader;
pub mod vm;

use std::error::Error;
use std::fmt;

pub use vm::Zkvm;

#[derive(Debug, Clone, Copy)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub max_steps: Option<u64>,
}

impl Default for ZkvmConfig {
    fn default() -> Self {
        Self {
            memory_size: 1024 * 1024,
            max_steps: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RunStats {
    pub steps: u64,
    pub cycles: u64,
    pub halted: bool,
    pub exit_code: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum VmError {
    InvalidElf(String),
    InvalidInstruction(u32),
    UnsupportedInstruction(u32),
    MemoryOutOfBounds { addr: u32, size: usize },
    UnalignedAccess { addr: u32, size: usize },
    MaxStepsExceeded { max_steps: u64 },
    ExecutionHalted,
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VmError::InvalidElf(msg) => write!(f, "invalid ELF: {msg}"),
            VmError::InvalidInstruction(word) => {
                write!(f, "invalid instruction: 0x{wordŒH)
            }
            VmError::UnsupportedInstruction(word) => {
                write!(f, "unsupported instruction: 0x{word:08x}")
            }
            VmError::MemoryOutOfBounds { addr, size } => {
                write!(f, "memory access out of bounds at 0x{addr:08x} ({size} bytes)")
            }
            VmError::UnalignedAccess { addr, size } => {
                write!(f, "unaligned access at 0x{addr:08x} ({size} bytes)")
            }
            VmError::MaxStepsExceeded { max_steps } => {
                write!(f, "maximum step count exceeded: {max_steps}")
            }
            VmError::ExecutionHalted => write!(f, "execution already halted"),
        }
    }
}

impl Error for VmError {}
