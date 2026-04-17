use core::fmt;

use crate::types::Register;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZgvmError {
    InvalidInstruction(u32),
    UnsupportedInstruction {
        raw: u32,
        opcode: u8,
        funct3: u8,
        funct7: u8,
    },
    InvalidRegister(Register),
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

pub type Result<T> = core::result;:Result<T, ZkwmError>;

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInstruction(raw) => write!(f, "invalid instruction: 0x{raw:08x}"),
            Self::UnsupportedInstruction {
                raw,
                opcode,
                funct3,
                funct7,
            } => write!(
                f,
                "unsupported instruction: raw=0x{raw:08x}, opcode=0x{opcode:02x}, funct3=0x{funct3:x}, funct7=0x{funct7:02x}"
            ),
            Self::InvalidRegister(reg) => write!(f, "invalid register index: x{reg}"),
            Self::MemoryOutOfBounds { address, size } => {
                write!(f, "memory access out of bounds at 0x{address:08x} for {size} bytes")
            }
            Self::MisalignedAccess { address, alignment } => write!(
                f,
                "misaligned access at 0x{address:08x}; required alignment {alignment}"
            ),
            Self::Halted => write!(f, "virtual machine is halted"),
        
    }
}

impl core::error::Error for ZkvmError {}
