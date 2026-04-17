#![forbid(unsafe_code)]

pub mod extensions;

mod bitfield;
mod decode;
mod error;
mod instruction;

pub use crate::decode::decode;
pub use crate::error::ZkvmError;
pub use crate::instruction::Instruction;

#[derive(Debug, Default, Clone, Copy)]
pub struct Zkvm {}

impl Zkvm {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn decode(&self, word: u32) -> Result<Instruction, ZkvmError> {
        crate::decode::decode(word)
    }
}
