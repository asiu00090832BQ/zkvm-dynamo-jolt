pub use crate::error::ZkvmError;

use crate::{decode, instruction::Instruction};

#[derive(Clone, Copy, Debug, Default)]
pub struct Zkvm;

impl Zkvm {
    pub const fn new() -> Self {
        Self
    }

    pub fn decode(&self, word: u32) -> Result<Instruction, ZkvmError> {
        decode::decode(word)
    }
}
