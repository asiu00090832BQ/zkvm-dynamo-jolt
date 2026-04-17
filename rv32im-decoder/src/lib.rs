#![no_std]

pub mod decoder;
pub mod encoding;
pub mod error;
pub mod instruction;

pub use decoder::{m_extension::MulDecomposition, Decoder};
pub use error::ZkvmError;
pub use instruction::{Instruction, MulVariant, Register};

pub struct Zkvm;

impl Zkvm {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode(word: u32) -> Result<Instruction, ZkvmError> {
        Decoder::decode(word)
    }

    pub fn decode_bytes(bytes: [u8; 4]) -> Result<Instruction, ZkvmError> {
        Self::decode(u32::from_le_bytes(bytes))
    }
}
