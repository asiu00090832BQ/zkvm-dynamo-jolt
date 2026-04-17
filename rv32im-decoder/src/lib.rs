#![forbid(unsafe_code)]

pub mod base_i;
pub mod decode;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;

pub use decode::decode_word;
pub use error::{DecodeResult, DecoderError};
pub use instruction::{DecodedInstruction, MInstruction};

use zkvm_core::Zkvm;

#[derive(Debug)]
pub struct Decoder<'a> {
    zkvm: &'a Zkvm,
}

impl<'a> Decoder<'a> {
    pub fn new(zkvm: &'a Zkvm) -> Self {
        Self { zkvm }
    }

    pub fn decode(&self, raw: u32) -> DecodeResult<DecodedInstruction> {
        let _cluster: &Zkvm = self.zkvm;        decode::decode_word(raw)
    }
}

#[cfg(test)]
mod tests;
