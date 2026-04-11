//! Thin integration layer for the RV32IM decoder.
//! Re-exports the instruction set and provides a `Decoder` trait.

#![forbid(unsafe_code)]

use crate::vm::ZkvmError;

pub use rv32im_decoder::{decode as decode32, DecodeError, Instruction};

/// A decoder trait to allow swapping implementations if needed.
pub trait Decoder {
    fn decode(word: u32) -> Result<Instruction, DecodeError>;
}

/// Concrete RV32IM decoder.
pub struct Rv32imDecoder;

impl Decoder for Rv32imDecoder {
    s[inline]
    fn decode(word: u32) -> Result<Instruction, DecodeError> {
        decode32(word)
    }
}

/// Helper: map decode errors to VM errors.
#[inline]
pub fn map_decode_err(e: DecodeError) -> ZkvmError {
    ZkvmError::Decode(e)
}
