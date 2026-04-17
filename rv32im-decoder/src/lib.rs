#![forbid(unsafe_code)]

pub mod decoder;
pub mod instruction;
pub mod selectors;

use core::fmt;

pub use decoder::Decoder;
pub use instruction::{
    BType, BranchOp, CsrOp, CsrType, FenceFields, IType, Instruction, JType, LoadOp,
    MulReduction16, Op, OpImm, RType, SType, StoreOp, UType,
};
pub use selectors::HierSelectors;

pub type DecodeResult<T> = Result<T, ZkvmError>;

pub trait Zkvm {
    fn decode(&self, word: u32) -> DecodeResult<Instruction>;

    fn decode_with_selectors(&self, word: u32) -> DecodeResult<(Instruction, HierSelectors)> {
        let instruction = self.decode(word)?;
        let selectors = HierSelectors::from_instruction(&instruction);
        Ok((instruction, selectors))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZkvmError {
    InvalidOpcode { word: u32, opcode: u8 },
    InvalidFunct3 { word: u32, opcode: u8, funct3: u8 },
    InvalidFunct7 { word: u32, opcode: u8, funct3: u8, funct7: u8 },
    InvalidFunct12 { word: u32, funct12: u16 },
    InvalidShiftEncoding { word: u32, funct7: u8 },
}

impl fmt::Display for ZkvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOpcode { word, opcode } => {
                write!(f, "invalid opcode 0x{opcode:02x} in word 0x{word:08x}")
            }
            Self::InvalidFunct3 {
                word,
                opcode,
                funct3,
            } => {
                write!(
                    f,
                    "invalid funct3 0b{funct3:03b} for opcode 0x{opcode:02x} in word 0x{word:08x}"
                )
            }
            Self::InvalidFunct7 {
                word,
                opcode,
                funct3,
                funct7,
            } => {
                write!(
                    f,
                    "invalid funct7 0b{funct7:07b} for opcode 0x{opcode:02x}, funct3 0b{funct3:03b} in word 0x{word:08x}"
                )
            }
            Self::InvalidFunct12 { word, funct12 } => {
                write!(
                    f,
                    "invalid funct12 0x{funct12:03x} in SYSTEM word 0x{word:08x}"
                )
            }
            Self::InvalidShiftEncoding { word, funct7 } => {
                write!(
                    f,
                    "invalid shift encoding with funct7 0b{funct7:07b} in word 0x{word:08x}"
                )
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ZkvmError {}
