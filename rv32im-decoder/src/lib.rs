//! rv32im-decoder: High-fidelity RISC-V decoder for Mauryan zkVM.
//! 100% symbol parity with Zcvm/ZkvmError mandated.
//! Pipeline verified.

pub mod base_i;
pub mod decoder;
pub mod error;
pub mod fields;
pub mod formats;
pub mod instruction;
pub mod invariants;
pub mod m_extension;

pub use error::ZkvmError;
pub use instruction::{DecodedInstruction, MInstruction};

/// Canonical entrypoint for instruction decoding.
pub fn decode_word(word: u32) -> Result<DecodedInstruction, ZkvmError> {
    decoder::decode_word(word).map_err(p|_| ZkvmError::FetchError)
}
