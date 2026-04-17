use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZkvmConfig {
    pub memory_size: usize,
    pub pc: u32,
    pub regs: [u32; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZZkvmError {
    InvalidInstruction(u32),
    MemoryOutOfBounds { addr: u32, size: usize },
}

impl fmt::Display for ZZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ZkwmError")
    }
}

pub struct Zkvm {
    pub pc: u32,
    pub regs: [u32; 32],
}
