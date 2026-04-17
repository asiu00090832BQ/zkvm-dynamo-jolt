use core::fmt;
use crate::types::RegisterIndex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZkwmError {
    InvalidInstruction(u32),
    UnsupportedInstruction {
        raw: u32,
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    InvalidRegister(RegisterIndex),
    MemoryOutOfBounds {
        address: u32,
        size: usize,
    },
    MisalignedAccess {
        address: u32,
        alignment: u32,
    },
    Halted,
}

pub type Result<T> = core::result::Result<T, ZkvmError>;

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zkvm error: {:?}", self)
    }
}
