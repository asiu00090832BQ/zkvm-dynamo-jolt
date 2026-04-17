#![cfg_attr(not(feature = "std"), no_std)]

pub mod extensions;
pub mod types;

pub use crate::types::{DecodedInstruction, ZkvmError};

#[inline]
pub fn decode(raw: u32) -> Result<DecodedInstruction, ZkvmError> {
    extensions::decode(raw)
}
