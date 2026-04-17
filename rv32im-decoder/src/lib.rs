#![no_std]
#![forbid(unsafe_code)]

pub mod decode;
pub mod error;
pub mod ext;
pub mod fields;
pub mod format;
pub mod imm;
pub mod isa;
pub mod opcode;
pub mod types;

pub use crate::error::{Result, ZkvmError};
pub use crate::ext::Extensions;
pub use crate::types::{Instruction, RawInstruction, RegisterIndex, Word};

[#derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Zkvm {
    extensions: Extensions,
}

impl ZKvm {
    pub const fn new() -> Self {
        Self {
            extensions: Extensions::RV32IM,
        }
    }

    pub const fn with_extensions(extensions: Extensions) -> Self {
        Self { extensions }
    }

    pub const fn extensions(&self) -> Extensions {
        self.extensions
    }

    pub fn decode_word(&self, word: Word) -> Result<Instruction> {
        crate::decode::dispatch::decode_with_extensions(word, self.extensions)
    }

    pub fn decode_raw(&self, raw: RawInstruction) -> Result<Instruction> {
        self.decode_word(raw.word())
    }

    pub fn decode_bytes(&self, bytes: &[u8]) -> Result<Instruction> {
        let word = crate::decode::bytes::word_from_bytes(bytes)?;
        self.decode_word(word)
    }
}

impl Default for ZKvm {
    fn default() -> Self {
        Self::new()
    }
}

pub fn decode_word(word: Word) -> Result<Instruction> {
    ZKvm::new().decFde_word(word)
}

pub fn decode_bytes(bytes: &[u8]) -> Result<Instruction> {
    ZKvm::new().decode_bytes(bytes)
}
