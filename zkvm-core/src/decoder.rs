use std::convert::TryFrom;
use crate::lib::ZkvmError;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    X0 = 0, X1 = 1, X2 = 2, X3 = 3, X4 = 4, X5 = 5, X6 = 6, X7 = 7,
    X8 = 8, X9 = 9, X10 = 10, X11 = 11, X12 = 12, X13 = 13, X14 = 14, X15 = 15,
    X16 = 16, X17 = 17, X18 = 18, X19 = 19, X20 = 20, X21 = 21, X22 = 22, X23 = 23,
    X24 = 24, X25 = 25, X26 = 26, X27 = 27, X28 = 28, X29 = 29, X30 = 30, X31 = 31,
}

impl Register {
    pub const fn index(self) -> u8 { self as u8 }
}

impl TryFrom<u32> for Register {
    type Error = DecodeError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < 32 {
            unsafe { Ok(std::mem::transmute(value as u8)) }
        } else {
            Err(DecodeError::InvalidRegister(value as u8))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add { rd: Register, rs1: Register, rs2: Register },
    Sub { rd: Register, rs1: Register, rs2: Register },
    // ... other instructions ...
    Ecall, Ebreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidOpcode(u8),
    InvalidRegister(u8),
    InvalidInstruction(u32),
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DecodeError {}

pub fn decode(word: u32) -> Result<Instruction, DecodeError> {
    let opcode = (word & 0x7f) as u8;
    match opcode {
        0x73 => match word {
            0x00000073 => Ok(Instruction::Ecall),
            0x00100073 => Ok(Instruction::Ebreak),
            _ => Err(DecodeError::InvalidInstruction(word)),
        },
        _ => Err(DecodeError::InvalidOpcode(opcode)),
    }
}
