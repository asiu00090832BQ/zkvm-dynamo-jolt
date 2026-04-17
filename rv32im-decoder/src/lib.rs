#![forbid(unsafe_code)]

pub mod base_i;
pub mod decoder;
pub mod error;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;

pub use decoder::decode_word;
pub use error:{DecodeResult, DecoderError};
pub use instruction::{DecodedInstruction, MInstruction};

#[cfg(test)]
mod tests;
